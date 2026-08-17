use std::io::Write;
use std::path::Path;
use std::{fs, io};


use zeroize::Zeroizing;

use crate::read_write::{PasswordBlock, SearchTarget, UnlockedVault, init_vault};
use crate::config::{Settings};
use crate::encryption::get_secure_input;

fn load_config() -> Result<Settings, (String, bool)> {
    match Settings::load() {
        Ok(value) => {
            if !value.1 {
                println!("Could not load previous settings, so loaded default ones")
            }
            Ok(value.0)
        },
        Err(e) => {
            Err((e.to_string(),false))
        }
    }
}

fn get_secret(message: &str) -> Result<Zeroizing<String>, (String, bool)> {
    match get_secure_input(&message) {
        Ok(val) => Ok(val),
        Err(e) => {
            Err((e.to_string(), false))
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

pub fn init_myvault() -> Result<(String, String), String> {
    
    let settings_string:String;

    let settings: Settings = match  Settings::load() {
        Ok(val) => {
            if val.1 == true {
                settings_string = "Succesfully loaded previous config file".to_string();
            } else {
                settings_string = "No existing config file, generated the default one".to_string();
            }
            val.0
        },
        Err(e) => return Err(e),
    };
    let str_filepath = format!("{}/.vault",&settings.vault_dir);
    if Path::new(&str_filepath).exists() {
        return Err("Vault already initiated".to_string())
    }
    
    if let Some(parent) = Path::new(&str_filepath).parent() {
        if !parent.as_os_str().is_empty() {
            fs::create_dir_all(parent)
                .map_err(|e| format!("Could not create parent directories: {}", e))?;
        }
    }


    init_vault(&str_filepath)?;

    Ok((settings_string, format!("Vault init succesfull, located in {}",&str_filepath)))
}

pub fn add_password(
    service: Option<String>,
    username: Option<String>,
) -> Result<String, (String,bool /* WHEN TRUE INTENTIONAL EXIT, WHEN FALSE PROCESS ERROR */)> {

    let search_target = match (&service, &username) {
        (Some(s), Some(u)) => SearchTarget::Both { service: s, username: u },
        _ => return Err(("You must provide a service or username argument \n -s (-service) \n -u (-username)".to_string(), true)),
    };

    let pw: Zeroizing<String>= get_secret("[KEY]")?;

    let settings = load_config()?;

    let mut unlocked_vault = UnlockedVault::open(&pw, &settings).map_err(|e| (e.to_string(), false))?;

    match unlocked_vault.resolve_target(search_target) {
        Ok(_) => {
            //Handle input and overriding later
            if ask_question("Duplicate Password meta found \n Do you want to overwrite it".to_string()).map_err(|e| (e.to_string(), false))? {
                let password_block = PasswordBlock {
                    service: service.unwrap(),
                    username: username.unwrap(),
                    password: get_secret("Type password you want to store")?,
                };

                unlocked_vault.remove_password(&password_block.service, &password_block.username).map_err(|e| (e.to_string(), false))?;
                unlocked_vault.add_password(password_block);
                unlocked_vault.save().map_err(|e| (e.to_string(), false))?;

                return Ok("Password Entry Succesfully overwritten".to_string());
            }
            Err(("Not overwriting password".to_string(), true))
            
        },
        Err((message, option)) => {
            if option.is_none() {
                let password_block = PasswordBlock {
                    service: service.unwrap(),
                    username: username.unwrap(),
                    password: get_secret("Type password you want to store")?,
                };

                unlocked_vault.add_password(password_block);
                unlocked_vault.save().map_err(|e| (e.to_string(), false))?;
                return Ok("Password succesfully added to vault".to_string());
            } 
            Err((message, true))
        }
    }
}

pub fn remove_password(
    service: Option<String>,
    username: Option<String>,
) -> Result<String, (String, bool /* WHEN TRUE INTENTIONAL EXIT, WHEN FALSE PROCESS ERROR */)> {

    let search_target = match (&service, &username) {
        (Some(s), Some(u)) => SearchTarget::Both { service: s, username: u },
        (None, Some(u)) => SearchTarget::Username(u),
        (Some(s), None) => SearchTarget::Service(s),
        (None, None) => return Err(("You must provide a service or username argument \n -s (-service) \n -u (-username)".to_string(), true)),
    };

    let pw = get_secret("[KEY]")?;

    let settings = load_config()?;

    let mut unlocked_vault = UnlockedVault::open(&pw, &settings).map_err(|e|(e.to_string(), false))?;
    let to_delete = match unlocked_vault.resolve_target(search_target) {
        Ok(t) => t,
        Err(e) => if e.1.is_some() {
            return Err(("Multiple matches found, you must provide a service or username argument \n -s (-service) \n -u (-username)".to_string(), true));
        } else {
            return Err((e.0.to_string(), true));
        }
    };

    if ask_question(format!("Are you sure you want to delete: \n service: {}\n username: {}\n",to_delete.0, to_delete.1)).map_err(|e|(e.to_string(), false))? {
        unlocked_vault.remove_password(&to_delete.0, &to_delete.1)
        .map_err(|e|(e.to_string(), false))?;
        unlocked_vault.save()
        .map_err(|e|(e.to_string(), false))?;
        Ok("Password removed succesfully from vault".to_string())
    } else {
        Err(("Aborted password remove".to_string(), true))
    }  
}