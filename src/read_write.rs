use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::Path;
use zeroize::{Zeroize, ZeroizeOnDrop, Zeroizing};
use crate::config::Settings;
use crate::encryption::{decrypt_bytes, derive_key, encrypt_toml};
use rand::rngs::OsRng;
use rand::RngCore;
use serde::{Deserialize, Serialize, Serializer};
use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;

fn serialize_zeroizing_string<S>(val: &Zeroizing<String>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(&**val)
}

#[derive(Serialize, Deserialize, Debug, Zeroize, ZeroizeOnDrop, Clone)]
pub struct PasswordBlock {
    pub service: Zeroizing<String>,
    pub username: Zeroizing<String>,
    #[serde(serialize_with = "serialize_zeroizing_string")]
    pub password: Zeroizing<String>,
}

// Top-level TOML table wrapper
#[derive(Serialize, Deserialize, Debug, Default, Zeroize, ZeroizeOnDrop)]
pub struct VaultData {
    pub auth: String,
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

pub fn init_vault(filepath: &str, pw: Zeroizing<String>, settings: Settings) -> Result<[u8; 16], String> {
    let mut salt: [u8; 16] = [0u8; 16];
    OsRng.fill_bytes(&mut salt);

    let key: Zeroizing<[u8; 32]> = derive_key(&pw, &salt, &settings).map_err(|e| e.to_string())?;

    let initial_data = VaultData {
        auth: "VALID_VAULT".to_string(),
        vec_passwords_blocks: vec![],
    };

    let toml_string = Zeroizing::new(
        toml::to_string(&initial_data)
            .map_err(|e| format!("Could not serialize initial vault: {}", e))?
    );

    let (nonce, encrypted_payload) = encrypt_toml(&toml_string, &key)
        .map_err(|e| e.to_string())?;

    let mut to_write: Vec<u8> = Vec::with_capacity(16 + 12 + encrypted_payload.len());
    to_write.extend_from_slice(&salt);
    to_write.extend_from_slice(&nonce);
    to_write.extend_from_slice(&encrypted_payload);

    atomically_write(filepath, &to_write)?;
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

pub fn check_auth(
    encrypted_txt: &[u8],
    nonce_bytes: &[u8; 12],
    key: &Zeroizing<[u8; 32]>
) -> Result<Zeroizing<VaultData>, String> {
    let decrypted: Zeroizing<String> = decrypt_bytes(nonce_bytes, key, encrypted_txt)?;

    let data: VaultData = toml::from_str(&decrypted)
        .map_err(|e| format!("Could not parse vault TOML: {}", e))?;

    if data.auth == "VALID_VAULT" {
        Ok(Zeroizing::new(data))
    } else {
        Err("Invalid password or corrupted vault auth marker".to_string())
    }
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
            return Err("No .vault found, try\nmyvault init \nChecking vault_dir in config".to_string());
        };

        if file_read.len() < 16 + 12 + 11 /*SALT+ NONCE + AUTH */ {
            return Err("File length too short, file corrupted".to_string());
        };

        let salt: [u8; 16] = file_read[0..16]
            .try_into()
            .map_err(|_| "Header too short for salt".to_string())?;

        let key: Zeroizing<[u8; 32]> = derive_key(master_password, &salt, settings)
            .map_err(|_| "Could not derive key".to_string())?;
        
        let nonce: [u8; 12] = file_read[16..28]
            .try_into()
            .map_err(|_| "Header too short for nonce".to_string())?;

        let ciphertext_slice: &[u8] = &file_read[28..];
        
        let data: Zeroizing<VaultData> = check_auth(ciphertext_slice, &nonce, &key)?;

        return Ok(Self {
            filepath,
            salt,
            key,
            data,
        });
    }
    pub fn resolve_target(
        &self,
        target: SearchTarget,
    ) -> Result<Zeroizing<PasswordBlock>, (String, Option<Vec<Zeroizing<PasswordBlock>>>)> {
        let matches: Vec<Zeroizing<PasswordBlock>> = self.data
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
            .map(|e| Zeroizing::new((*e).clone()))
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
    
    pub fn fuzzy_search(&self, query: &str) -> Result<Option<Vec<PasswordBlock>>, String> {
    if query.trim().is_empty() {
        return Err("Provide a query for the search".to_string());
    }

    let matcher = SkimMatcherV2::default();

    let mut scored_matches: Vec<(i64, PasswordBlock)> = self
        .data
        .vec_passwords_blocks
        .iter()
        .filter_map(|block| {
            let s_score = matcher.fuzzy_match(&block.service, query);
            let u_score = matcher.fuzzy_match(&block.username, query);

            let best_score = match (s_score, u_score) {
                (Some(s), Some(u)) => Some(s.max(u)),
                (Some(s), None) => Some(s),
                (None, Some(u)) => Some(u),
                (None, None) => None,
            }?;

            Some((best_score, block.clone()))
        })
        .collect();

    if scored_matches.is_empty() {
        return Ok(None);
    }

    // Sort descending by match score
    scored_matches.sort_by(|a, b| b.0.cmp(&a.0));

    // Extract sorted PasswordBlocks
    let sorted_blocks: Vec<PasswordBlock> = scored_matches
        .into_iter()
        .map(|(_, block)| block)
        .collect();

    Ok(Some(sorted_blocks))
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