use std::fs::{self, OpenOptions};
use std::io::{Write};
use std::path::Path;
use zeroize::{Zeroize, ZeroizeOnDrop, Zeroizing};
use crate::encryption::{decrypt_json, derive_key, encrypt_json};
use rand::rngs::OsRng;
use rand::RngCore;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Zeroize, ZeroizeOnDrop)]
pub struct PasswordBlock {
    pub service: String,
    pub username: String,
    pub password: String
}

fn atomically_write(filepath: &str, write: &[u8]) -> Result<(), String> {
    let tmp_path = format!("{}.tmp", filepath);

    let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(true)
            .open(&tmp_path)
            .map_err(|e| format!("Could not setup OpenOptions on the .tmp file: {}", e))?;

    file.write_all(write)
        .map_err(|_| "Could not write to .tmp".to_string())?;

    file.sync_all()
        .map_err(|_| "Could not sync_all .tmp file".to_string())?;

    drop(file);

    fs::rename(&tmp_path, filepath)
        .map_err(|_| "Could not update .tmp file to main .vault file".to_string())?;

    Ok(())
}

fn init_vault(filepath: &str) -> Result<[u8; 16] ,String> {
//Generate Salt
let mut salt=  [0u8; 16];

OsRng.fill_bytes(&mut salt);

//Create .vault and write salt to .vault file

atomically_write(filepath, &salt)
    .map_err(|e| e)?;

Ok(salt)
}




pub fn read_vault(filepath: &str, master_password:&Zeroizing<String> ) -> Result<([u8;16],Option<[u8;12]>,Zeroizing<String>), String> {

let file_read= if Path::new(&filepath).exists() {
    fs::read(&filepath)
        .map_err(|_| "Could not Read File".to_string())?
} else {
    let salt = init_vault(&filepath)
        .map_err(|e| e)?;
    return Ok((salt, None, Zeroizing::new("[]".to_string())));
};


//Min File length check
if file_read.len() < 16 {
    return Err("File length too short, file corrupted".to_string());
}

let salt: [u8; 16] = file_read[0..16].try_into().map_err(|_| "Header too short for salt".to_string())?;

//Only salt (Initialized) check
if file_read.len() == 16 {
    return Ok((salt, None, Zeroizing::new("[]".to_string())));
}

//Corrupted salt+nonce check
if file_read.len() < 28 {
    return Err("Vault Header corrupted or too short".to_string());
}

let key = derive_key(master_password, &salt).map_err(|_| "Could not derive key".to_string())?;
let nonce: [u8; 12] = file_read[16..28].try_into().map_err(|_| "Header too short for nonce".to_string())?;

let ciphertext_slice: &[u8] = &file_read[28..];

let decrypted_text = decrypt_json(&nonce, &key, ciphertext_slice)?;

Ok((salt, Some(nonce), decrypted_text))

}


pub fn append_password_block(filepath: &str, master_password:&Zeroizing<String>, input: PasswordBlock) -> Result<(), String> {
    //Read the existing vault
let vault_tuple = read_vault(filepath, master_password)
    .map_err(|_| "Could not read vault".to_string())?;

let salt = vault_tuple.0;

//Serialize the contnet into a Vec of structs
let mut raw_vec:Vec<PasswordBlock> = serde_json::from_str(&vault_tuple.2)
                                                .map_err(|_| "Could not deserialize vault content".to_string())?;

//Push the PasswordBlock Struct into the Vec
raw_vec.push(input);

//Pass ownership into a memory secure Zeroizing wraper
let zeroed_vec = Zeroizing::new(raw_vec);
let string_vault:Zeroizing<String> = Zeroizing::new(serde_json::to_string(&*zeroed_vec)
                                        .map_err(|_| "Could not to_string the Vault".to_string())?);
//Encrypt the zeroed_vec

let key = derive_key(master_password, &salt).map_err(|_| "Could not derive key".to_string())?;

let nonce_cipher_tuple = encrypt_json(&string_vault, &key)
                                                            .map_err(|_| "Could not encrypt write".to_string())?;

let mut vault_file   = Vec::with_capacity(16 + 12 + nonce_cipher_tuple.1.len()); //16 salt len + 12 nonce len + chiphertxt len

vault_file.extend_from_slice(&salt);
vault_file.extend_from_slice(&nonce_cipher_tuple.0);
vault_file.extend_from_slice(&nonce_cipher_tuple.1);


atomically_write(filepath, &vault_file)?;

Ok(())
}

