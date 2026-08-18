mod encryption;
mod read_write;
mod config;
mod cli;
mod clipboard;

use clap::{Parser, Subcommand};
use crate::cli::{add_password, copy_pw, init_myvault, list_vault, remove_password, reset_myvault};
use crate::clipboard::init_clipboard_daemon;

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
        #[arg(short, long)]
        service: Option<String>,

        #[arg(short, long)]
        username: Option<String>,
    },
    
    #[command(alias = "l")]
    List ,

    #[command(alias = "r")]
    Reset
}




fn main() {  
    init_clipboard_daemon();

    let cli = Cli::parse();

    let response = (|| -> Result<String, (String, i32)> {
        match cli.command {
            Commands::Init => {
                Ok(init_myvault()?)
            },
            Commands::Add { service, username } => {
                Ok(add_password(service, username)?)
            },
            Commands::Remove { service, username } => {
                Ok(remove_password(service, username)?)
            },
            Commands::Copy { service, username } => {
                Ok(copy_pw(service, username)?)
            },
            Commands::List => Ok(list_vault()?),
            Commands::Reset => Ok(reset_myvault()?),
            _ => Err(("Opsss".to_string(), 0))
        }
    })().map_err(|e|{
        (e.0, e.1)
    });

    match response {
        Ok(msg) => {
            println!("{}",msg);
            std::process::exit(0);
        },
        Err((msg, exit_code)) => {
            eprintln!("{}", msg);
            std::process::exit(exit_code);
        }
    }

}