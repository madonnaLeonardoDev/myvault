use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::Path;
use zeroize::{Zeroize, ZeroizeOnDrop, Zeroizing};
use crate::config::Settings;
use crate::encryption::{decrypt_json, derive_key, encrypt_toml};
use rand::rngs::OsRng;
use rand::RngCore;
use serde::{Deserialize, Serialize, Serializer};

fn serialize_zeroizing_string<S>(val: &Zeroizing<String>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(&**val)
}

#[derive(Serialize, Deserialize, Debug, Zeroize, ZeroizeOnDrop)]
pub struct PasswordBlock {
    pub service: String,
    pub username: String,
    #[serde(serialize_with = "serialize_zeroizing_string")]
    pub password: Zeroizing<String>,
}

// Top-level TOML table wrapper
#[derive(Serialize, Deserialize, Debug, Default, Zeroize, ZeroizeOnDrop)]
pub struct VaultData {
    pub vec_passwords_blocks: Vec<PasswordBlock>,
}

pub enum SearchTarget<'a> {
    Service(&'a str),
    Username(&'a str),
    Both { service: &'a str, username: &'a str },
}

pub fn atomically_write(filepath: &str, write: &[u8]) -> Result<(), String> {
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

pub fn init_vault(filepath: &str) -> Result<[u8; 16], String> {
    let mut salt = [0u8; 16];
    OsRng.fill_bytes(&mut salt);
    atomically_write(filepath, &salt)?;
    Ok(salt)
}

pub fn write_to_vault(
    filepath: &str,
    encryption_content: (&[u8; 16], &[u8; 12], &[u8]),
) -> Result<(), String> {
    let mut vault_file = Vec::with_capacity(16 + 12 + encryption_content.2.len());
    vault_file.extend_from_slice(encryption_content.0);
    vault_file.extend_from_slice(encryption_content.1);
    vault_file.extend_from_slice(encryption_content.2);

    atomically_write(filepath, &vault_file)?;
    Ok(())
}


pub struct UnlockedVault {
    filepath: String,
    salt: [u8; 16],
    key: Zeroizing<[u8;32]>,
    pub data: Zeroizing<VaultData>,
}

impl UnlockedVault {
    /// 1. Unlocks the vault and derives the key EXACTLY ONCE
    pub fn open(
        master_password: &Zeroizing<String>,
        settings: &Settings,
    ) -> Result<Self, String> {
        let filepath = format!("{}/.vault", settings.vault_dir);

        let file_read = if Path::new(&filepath).exists() {
            fs::read(&filepath).map_err(|_| "Could not Read File".to_string())?
        } else {
            return Err("No .vault found, try\n myvault init | Init Vault".to_string());
        };

        if file_read.len() < 16 {
            return Err("File length too short, file corrupted".to_string());
        };

        let salt: [u8; 16] = file_read[0..16]
            .try_into()
            .map_err(|_| "Header too short for salt".to_string())?;

        if file_read.len() == 16 {
            let key: Zeroizing<[u8; 32]> = derive_key(master_password, &salt, settings)
            .map_err(|_| "Could not derive key".to_string())?;


            return Ok(Self {
                filepath,
                salt,
                key,
                data: Zeroizing::new(VaultData::default()),
            });
        }

        if file_read.len() < 28 {
            return Err("Vault Header corrupted or too short".to_string());
        }

        let key: Zeroizing<[u8; 32]> = derive_key(master_password, &salt, settings)
            .map_err(|_| "Could not derive key".to_string())?;
        
        let nonce: [u8; 12] = file_read[16..28]
            .try_into()
            .map_err(|_| "Header too short for nonce".to_string())?;

        let ciphertext_slice: &[u8] = &file_read[28..];
        
        let decrypted_text: Zeroizing<String> = decrypt_json(&nonce, &key, ciphertext_slice)?;

        let data: Zeroizing<VaultData> = Zeroizing::new(
            toml::from_str::<VaultData>(&*decrypted_text)
                .map_err(|e| format!("Could not parse vault into TOML: {}", e))?
        );

        Ok(Self {
            filepath,
            salt,
            key,
            data,
        })
    }
    pub fn resolve_target(
        &self,
        target: SearchTarget,
    ) -> Result<Zeroizing<(String, String)>, (String, Option<Vec<Zeroizing<(String, String)>>>)> {
        let matches: Vec<Zeroizing<(String, String)>> = self.data
            .vec_passwords_blocks
            .iter()
            .filter(|e| match target {
                SearchTarget::Service(s) => e.service.eq_ignore_ascii_case(s),
                SearchTarget::Username(u) => e.username.eq_ignore_ascii_case(u),
                SearchTarget::Both { service, username } => {
                    e.service.eq_ignore_ascii_case(service)
                        && e.username.eq_ignore_ascii_case(username)
                }
            })
            .map(|e| Zeroizing::new((e.service.clone(), e.username.clone())))
            .collect();

        match matches.len() {
            0 => Err(("No matching entry found in vault".to_string(), None)),
            1 => Ok(matches.into_iter().next().unwrap()),
            _ => Err((
                "More than 1 match found, please specify with service and username".to_string(),
                Some(matches),
            )),
        }
    }

    /// 3. Add an entry to memory
    pub fn add_password(&mut self, input: PasswordBlock) {
        self.data.vec_passwords_blocks.push(input);
    }

    /// 4. Remove an entry from memory
    pub fn remove_password(&mut self, service: &str, username: &str) -> Result<(), String> {
        let index = self.data
            .vec_passwords_blocks
            .iter()
            .position(|e| {
                e.service.eq_ignore_ascii_case(service)
                    && e.username.eq_ignore_ascii_case(username)
            })
            .ok_or_else(|| "Could not find target entry in vault".to_string())?;

        self.data.vec_passwords_blocks.remove(index);
        Ok(())
    }
    pub fn save(&self) -> Result<(), String> {
        let string_vault: Zeroizing<String> = Zeroizing::new(
            toml::to_string(&self.data)
                .map_err(|e| format!("Could not to_string the Vault: {}", e))?,
        );

        let nonce_cipher_tuple = encrypt_toml(&string_vault, &self.key)
            .map_err(|_| "Could not encrypt write".to_string())?;

        write_to_vault(
            &self.filepath,
            (&self.salt, &nonce_cipher_tuple.0, &nonce_cipher_tuple.1),
        )?;

        Ok(())
    }
}