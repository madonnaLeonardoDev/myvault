mod encryption;
mod read_write;
mod config;
mod cli;
mod clipboard;

use clap::{CommandFactory, Parser, Subcommand};
use clap_complete::{generate, Shell};
use crate::cli::{add_password, config, copy_pw, init_myvault, list_vault, remove_password, reset_myvault, search};
use crate::clipboard::init_clipboard_daemon;

#[derive(Parser)]
#[command(
    name = "myvault",
    // Disable automatic --help, -h, and help subcommand
    disable_help_flag = true,
    disable_help_subcommand = true,
    // Disable automatic --version and -V flags
    disable_version_flag = true
)]
/// A secure, locally encrypted command-line password vault
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,

    /// Display help information for myvault or a specific command
    #[arg(short = 'h', long = "help", global = true)]
    pub help: bool,

    /// Print the current version of myvault
    #[arg(short = 'v', long = "version", global = true)]
    pub version: bool,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Initialize a new secure vault
    #[command(alias = "i")]
    Init,
    
    /// Copy a password to the system clipboard
    #[command(alias = "cp")]
    Copy {
        /// Target service or website to copy the password for
        #[arg(short, long)]
        service: Option<String>,

        /// Specific username to copy (if multiple exist for the service)
        #[arg(short, long)]
        username: Option<String>,
    },

    /// Add a new credential entry to the vault
    #[command(alias = "a")]
    Add {
        /// Target service or website to save
        #[arg(short, long)]
        service: Option<String>,

        /// Username or email associated with the service
        #[arg(short, long)]
        username: Option<String>,
    },

    /// Remove a credential entry from the vault
    #[command(alias = "rm")]
    Remove {
        /// Target service or website to delete
        #[arg(short, long)]
        service: Option<String>,

        /// Specific username to delete (if multiple exist for the service)
        #[arg(short, long)]
        username: Option<String>,
    },

    /// Search the vault for specific services or usernames
    #[command(alias = "s")]
    Search {
        /// Search query terms (captures all words typed after search)
        #[arg(num_args = 1.., allow_hyphen_values = true, required = true)]
        query: Vec<String>,
    },
    
    /// List all saved services stored in the vault
    #[command(alias = "l")]
    List,

    /// Delete the vault entirely and erase all credentials
    #[command(alias = "r")]
    Reset,

    /// Manage vault configuration and settings
    #[command(alias = "cfg")]
    Config,

    /// Generate autocomplete scripts for your shell
    #[command(hide = true)] // Hidden from main help output
    Completions {
        /// The target shell to generate completions for (zsh, bash, fish)
        #[arg(value_enum)]
        shell: Shell,
    },
}




fn main() {  
    init_clipboard_daemon();

    let cli = Cli::parse();

    if cli.version {
        println!("myvault v0.1.0");
        return;
    }

    if cli.help || cli.command.is_none() {
        let help_text = r#"myvault v1.0.0 - secure terminal password vault

USAGE:
  myvault <COMMAND> [OPTIONS]

VAULT LIFECYCLE:
  init, i               Initialize a new vault and derive master key
  reset, r              Destroy and re-initialize an existing vault

MANAGEMENT:
  add, a                Add a new service/username/password entry
  remove, rm            Delete an existing vault entry

RETRIEVAL:
  list, l               List all stored entry services and usernames
  search, s             Search entries by service name or username
  copy, cp              Decrypt and copy a password directly to clipboard

SYSTEM:
  config, cfg           Display config file path and active parameters

FLAGS:
  -h, -help             Show this help message
  -v, -version          Display version information
"#;

    print!("{}", help_text);
    return;
    }

    if let Some(Commands::Completions { shell }) = cli.command {
        let mut cmd = Cli::command();
        let bin_name = cmd.get_name().to_string();
        generate(shell, &mut cmd, bin_name, &mut std::io::stdout());
        return;
    }


    let response = (|| -> Result<String, (String, i32)> {
        match cli.command.unwrap() {
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
            Commands::Search { query } => {
                Ok(search(&query.join(" "))?)
            },
            Commands::Config => Ok(config()?),
            Commands::Completions { .. } => unreachable!()
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