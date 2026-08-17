use serde::{Deserialize, Serialize};
use std::fs::{self};
use std::path::{PathBuf};
use crate::read_write::atomically_write;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Settings{
    //Encryption Settings
    pub memory_cost: u32,
    pub time_cost: u32,
    pub threads_cost: u32,

    //Vaut Path settings

    pub vault_dir: String
}

impl Settings {
    pub fn default() -> Self {
        let default_vault_dir = dirs::data_local_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("myvault")
            .to_string_lossy()
            .into_owned();

        Self {
            memory_cost: 64 * 1024, // 64mb
            time_cost: 3, //3 passes
            threads_cost: 4, //utilizes 4 threads

            vault_dir: default_vault_dir
        }
    }
    
    fn validate(&self) -> Result<(), String> {
        if self.memory_cost < 8 * 1024 || self.memory_cost > 1024 * 1024 {
            return Err("Memory cost has to be between 8192KiB and 1048576KiB".to_string());
        }
        if self.time_cost < 1 || self.time_cost > 10 {
            return Err("Time cost has to be between 1 pass and 10 passes".to_string());
        }
        if self.threads_cost < 1 || self.threads_cost > 16 {
            return Err("Threads cost has to be between 1 and 16 passes".to_string());
        }
        Ok(())
    }

    pub fn save(&self) -> Result<(), String> {

    self.validate()?;
    
    let settings_toml_string:String = toml::to_string(self)
                                                .map_err(|_| "Could not to_string default settings".to_string())?;

    let config_dir = dirs::config_local_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("myvault");

    if !config_dir.exists() {
        fs::create_dir_all(&config_dir)
            .map_err(|_| "Could not create config dir".to_string())?;
    }
    atomically_write(&config_dir.join("config.toml").to_string_lossy(), settings_toml_string.as_bytes())?;

    Ok(())
    }

    pub fn set_default() -> Result<(), String> {
       Settings::default().save()?;

        Ok(())
    }

    pub fn load() -> Result<(Settings, bool /*if true it means that it has loaded existing config if false it generated default oness */), String> {
        let config_dir = dirs::config_local_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("myvault");

        if !config_dir.join("config.toml").exists() {
            let default_sett = Settings::default();
            default_sett.save()?;
            return Ok((default_sett, false));
        }
        let config_bytes_read = fs::read(&config_dir.join("config.toml"))
        .map_err(|_|{
                let _ = Settings::set_default();
                "Could not read config.toml, it has been reset to its default".to_string()
            })?;

        if config_bytes_read.is_empty() {
            Settings::set_default()?;
            return Err("Could not read config.toml as it appears empty".to_string());
        }

        let config_struc:Settings = toml::from_str(std::str::from_utf8(&config_bytes_read)
        .map_err(|_| "Could not get utf8 string from config read")?
        ).map_err(|_| "Could not get valid toml string from config string")?; 

        config_struc.validate()?;

        Ok((config_struc, true))
    }
}