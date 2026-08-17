mod encryption;
mod read_write;
mod config;
mod cli;

use clap::builder::Str;
use clap::{Parser, Subcommand};
use zeroize::Zeroizing;
use crate::cli::init_myvault;
use crate::config::Settings;
use crate::encryption::get_secure_input;
use crate::read_write::{PasswordBlock, SearchTarget, UnlockedVault};

#[derive(Parser)]
#[command(name = "myvault", author, version, about = "Secure Password Vault")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    #[command(alias = "i")]
    Init,
    
    #[command(alias = "cp")]
    Copy {
        #[arg(short, long)]
        service: Option<String>,

        #[arg(short, long)]
        username: Option<String>,
    },

    #[command(alias = "a")]
    Add {
        #[arg(short, long)]
        service: Option<String>,

        #[arg(short, long)]
        username: Option<String>,
    },

    #[command(alias = "rm")]
    Remove {
        #[arg(short, long)]
        service: Option<String>,

        #[arg(short, long)]
        username: Option<String>,
    },

    #[command(alias = "s")]
    Search {
        query: Option<String>,
    },
    
    #[command(alias = "l")]
    List 
}

fn load_config() -> Settings {
    match Settings::load() {
        Ok(value) => {
            if !value.1 {
                println!("Could not load previous settings, so loaded default ones")
            }
            value.0
        },
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    }
}

fn get_secret(message: String) -> Zeroizing<String> {
    match get_secure_input(&message) {
        Ok(val) => val,
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    }
}


fn main() {  
    let cli = Cli::parse();

    
}