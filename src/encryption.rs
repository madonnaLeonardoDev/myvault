use rpassword::prompt_password;
use std::io;
use zeroize::Zeroizing;
use argon2::{Algorithm, Argon2, Params, Version};
use aes_gcm::{
    Aes256Gcm, Nonce, aead::{Aead, KeyInit}
};
use rand::rngs::OsRng;
use rand::RngCore;

//get a memory sanitized safe password input
pub fn get_secure_input(prompt: &str) -> io::Result<Zeroizing<String>> {
    let input = prompt_password(prompt)?;

    Ok(Zeroizing::new(input))
}

//master pw hashing using password + salt

pub fn derive_key(
    master_password: &str,
    salt:&[u8; 16],
) -> Result<Zeroizing<[u8; 32]>, String> {
    let mut key = Zeroizing::new([0u8; 32]);

    let params = Params::new(19456, 2, 1, Some(32)).map_err(|e| e.to_string())?;
    let argon2 = Argon2::new(Algorithm::Argon2id, Version::V0x13, params);

    argon2
        .hash_password_into(master_password.as_bytes(), salt, key.as_mut())
        .map_err(|_| "Could not Hash Password".to_string())?;

    Ok(key)
}

pub fn encrypt_json(
  json: &str,
  key: &Zeroizing<[u8; 32]> 
) -> Result<([u8; 12], Vec<u8>), String> {
    let mut nonce_bytes = [0u8; 12];

    OsRng.fill_bytes(&mut nonce_bytes);

    let nonce = Nonce::from_slice(&nonce_bytes);

    let chiper = Aes256Gcm::new_from_slice(key.as_ref())
    .map_err(|_|"Failed createing the Cypher from key".to_string())?;

    let chiphertext = chiper.encrypt(nonce, json.as_bytes())
        .map_err(|_| "Failed Encrypting the JSON".to_string())?;

    Ok((nonce_bytes, chiphertext))

}

pub fn decrypt_json(
    nonce_bytes: &[u8; 12],
    key: &Zeroizing<[u8; 32]>,
    ciphertext: &[u8]
) -> Result<Zeroizing<String>, String> {

    let key = Aes256Gcm::new_from_slice(key.as_ref())
        .map_err(|_| "Could not get Aes256Gcm key from slice".to_string())?;

    let nonce = Nonce::from_slice(nonce_bytes);

    let decrypted_plain_txt = Zeroizing::new(
        String::from_utf8(
            key.decrypt(&nonce, ciphertext.as_ref())
        .map_err(|_|"Could not decrypt chyphertext, either wrong ker or corrupted".to_string())?
        )
        .map_err(|_| "Could not convert Vec[u8] into String".to_string())?
    );

    Ok(decrypted_plain_txt)

}