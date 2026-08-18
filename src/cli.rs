use std::io::Write;
use std::path::Path;
use std::{fs, io};
use zeroize::Zeroizing;

use crate::read_write::{PasswordBlock, SearchTarget, UnlockedVault, init_vault};
use crate::config::{Settings};
use crate::encryption::get_secure_input;
use crate::clipboard::copy_and_persist_clipboard;

fn load_config() -> Result<Settings, (String, i32)> {
    match Settings::load() {
        Ok(value) => {
            if !value.1 {
                println!("Could not load previous settings, so loaded default ones")
            }
            Ok(value.0)
        },
        Err(e) => {
            Err((e.to_string(),1))
        }
    }
}

fn get_secret(message: &str) -> Result<Zeroizing<String>, (String, i32)> {
    match get_secure_input(&message) {
        Ok(val) => Ok(val),
        Err(e) => {
            Err((e.to_string(), 1))
        }
    }
}

fn ask_question(message: String) -> Result<bool, String> {
    print!("{} [Y/n]   ", message);

    io::stdout().flush().map_err(|e| e.to_string())?;

    let mut input = String::new();

    io::stdin().read_line(&mut input).map_err(|e| e.to_string())?;

    match input.trim().to_lowercase().as_str() {
        "yes" | "y" | "" => {
            Ok(true)
        },
        _=> {
            Ok(false)
        }
    }
}

pub fn init_myvault() -> Result<String, (String, i32)> {

    let settings: Settings = load_config()?;
    let str_filepath = format!("{}/.vault",&settings.vault_dir);
    if Path::new(&str_filepath).exists() {
        return Err(("Vault already initiated".to_string(), 0))
    }
    
    if let Some(parent) = Path::new(&str_filepath).parent() {
        if !parent.as_os_str().is_empty() {
            fs::create_dir_all(parent)
                .map_err(|e| (format!("Could not create parent directories: {}", e), 1))?;
        }
    }

    let settings = load_config()?;
    let pw = get_secret("Insert the key for this vault \n Once set it cannot be changed without erasing the vault \n")?;

    init_vault(&str_filepath, pw, settings)
        .map_err(|e| (e, 1))?;

    Ok(format!("Vault init succesfull, located in {}",&str_filepath))
}

pub fn reset_myvault() -> Result<String, (String, i32)> {
    let settings = load_config()?;

    let file_path = format!("{}/.vault",&settings.vault_dir);

    if !Path::new(&file_path).exists() {
        return Err(("No existing .vault initiated \n check config or myvault init".to_string(), 0));
    }

    if ask_question("Resetting the vault means losing access to all its data currently stored, proceed?".to_string())
    .map_err(|e| (e, 1))? {
     fs::remove_file(&file_path)
        .map_err(|e| (format!("Could not remove old vault: {}", e), 1))?;
     println!("Old vault removed initiating new one");
     let settings = load_config()?;
     let pw = get_secret("Insert the key for new vault \n Once set it cannot be changed without erasing the vault \n")?;
     init_vault(&file_path, pw, settings)
     .map_err(|e|(e, 1))?;  
    } else {
        return Err(("Vault reset cancelled".to_string(), 0))
    }

    Ok(format!("Vault succesfully reset \n PATH: {}", &settings.vault_dir) )
}

pub fn add_password(
    service: Option<String>,
    username: Option<String>,
) -> Result<String, (String,i32 /* PROCESS EXIT CODE */)> {

    let search_target = match (&service, &username) {
        (Some(s), Some(u)) => SearchTarget::Both { service: s, username: u },
        _ => return Err(("You must provide a service or username argument \n -s (-service) \n -u (-username)".to_string(), 0)),
    };

    let pw: Zeroizing<String>= get_secret("[KEY]")?;

    let settings = load_config()?;

    let mut unlocked_vault = UnlockedVault::open(&pw, &settings).map_err(|e| (e.to_string(), 1))?;

    match unlocked_vault.resolve_target(search_target) {
        Ok(_) => {
            //Handle input and overriding later
            if ask_question("Duplicate Password meta found \n Do you want to overwrite it".to_string()).map_err(|e| (e.to_string(), 1))? {
                let password_block = PasswordBlock {
                    service: Zeroizing::new(service.unwrap()),
                    username: Zeroizing::new(username.unwrap()),
                    password: get_secret("Type password you want to store")?,
                };

                unlocked_vault.remove_password(&password_block.service, &password_block.username).map_err(|e| (e.to_string(), 1))?;
                unlocked_vault.add_password(password_block);
                unlocked_vault.save().map_err(|e| (e.to_string(), 1))?;

                return Ok("Password Entry Succesfully overwritten".to_string());
            }
            Err(("Not overwriting password".to_string(), 0))
            
        },
        Err((message, option)) => {
            if option.is_none() {
                let password_block = PasswordBlock {
                    service: Zeroizing::new(service.unwrap()),
                    username: Zeroizing::new(username.unwrap()),
                    password: get_secret("Type password you want to store")?,
                };

                unlocked_vault.add_password(password_block);
                unlocked_vault.save().map_err(|e| (e.to_string(), 1))?;
                return Ok("Password succesfully added to vault".to_string());
            } 
            Err((message, 0))
        }
    }
}

pub fn remove_password(
    service: Option<String>,
    username: Option<String>,
) -> Result<String, (String, i32 /* PROCESS EXIT CODE */)> {

    let search_target = match (&service, &username) {
        (Some(s), Some(u)) => SearchTarget::Both { service: s, username: u },
        (None, Some(u)) => SearchTarget::Username(u),
        (Some(s), None) => SearchTarget::Service(s),
        (None, None) => return Err(("You must provide a service or username argument \n -s (-service) \n -u (-username)".to_string(), 0)),
    };

    let pw = get_secret("[KEY]")?;

    let settings = load_config()?;

    let mut unlocked_vault = UnlockedVault::open(&pw, &settings).map_err(|e|(e.to_string(), 1))?;
    let to_delete = match unlocked_vault.resolve_target(search_target) {
        Ok(t) => t,
        Err(e) => if e.1.is_some() {
            return Err(("Multiple matches found, you must provide a service or username argument \n -s (-service) \n -u (-username)".to_string(), 0));
        } else {
            return Err((e.0.to_string(), 0));
        }
    };

    if ask_question(format!("Are you sure you want to delete: \n service: {}\n username: {}\n",&to_delete.service.as_str(), &to_delete.username.as_str())).map_err(|e|(e.to_string(), 1))? {
        unlocked_vault.remove_password(&to_delete.service, &to_delete.username)
        .map_err(|e|(e.to_string(), 1))?;
        unlocked_vault.save()
        .map_err(|e|(e.to_string(), 1))?;
        Ok("Password removed succesfully from vault".to_string())
    } else {
        Err(("Aborted password remove".to_string(), 0))
    }  

}

pub fn copy_pw  (
    service: Option<String>,
    username: Option<String>
) -> Result<String, (String, i32)> {
    let search_target = match (&service, &username) {
        (Some(s), Some(u)) => SearchTarget::Both { service: s, username: u },
        (None, Some(u)) => SearchTarget::Username(u),
        (Some(s), None) => SearchTarget::Service(s),
        (None, None) => return Err(("You must provide a service or username argument \n -s (-service) \n -u (-username)".to_string(), 0)),
    };

    let pw = get_secret("[KEY]")?;

    let settings = load_config()?;

    let unlocked_vault = UnlockedVault::open(&pw, &settings)
        .map_err(|e|(e.to_string(), 1))?;
    let to_copy_pw: Zeroizing<String> = match unlocked_vault.resolve_target(search_target) {
        Ok(t) => t.password.clone(),
        Err(e) => if e.1.is_some() {
            return Err(("Multiple matches found, you must provide a service or username argument \n -s (-service) \n -u (-username)".to_string(), 0));
        } else {
            return Err((e.0.to_string(), 0));
        }
    };

    unlocked_vault.save()
        .map_err(|e| (e, 1))?;

    let cp_duration:u64 = 15;

    match copy_and_persist_clipboard(&to_copy_pw, cp_duration) {
        Ok(m) => Ok(m),
        Err(e) => Err((e.to_string(), 1))
    }
}

pub fn list_vault() -> Result<String, (String, i32)> {
    let settings = load_config()?;

    let pw = get_secret("[KEY]")?;

    let unlocked_vault = UnlockedVault::open(&pw, &settings)
        .map_err(|e|(e, 1))?;

    let mut max_service_len = "SERVICE".len();
    let mut max_username_len = "USERNAME".len();

    let mut service_vec:Vec<&Zeroizing<String>> = Vec::new();
    let mut username_vec: Vec<&Zeroizing<String>> = Vec::new();

    for e in &unlocked_vault.data.vec_passwords_blocks {
        max_service_len = max_service_len.max(e.service.len());
        max_username_len = max_username_len.max(e.username.len());

        service_vec.push(&e.service);
        username_vec.push(&e.username);
    };

    //PRINT HEADER

    println!(
            "| {:<s_len$} | {:<u_len$} | PASSWORD",

            "SERVICE",
            "USERNAME",
            s_len = max_service_len,
            u_len = max_username_len
        );

    for e in &unlocked_vault.data.vec_passwords_blocks {
        println!(
            "| {:<s_len$} | {:<u_len$} | ***",

            &*e.service,
            &*e.username,
            s_len = max_service_len,
            u_len = max_username_len
        );
    };

    unlocked_vault.save()
        .map_err(|e| (e, 1))?;
    Ok("".to_string())
}