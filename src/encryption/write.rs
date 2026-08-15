use rpassword::prompt_password;
use std::io;
use zeroize::Zeroizing;
use argon2::{Algorithm, Argon2, Params, Version};


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
        .map_err(|e| e.to_string())?;

    Ok(key)
}

