use arboard::Clipboard;
use std::{env, process::{Command, Stdio}, thread, time::Duration};

pub fn copy_and_persist_clipboard(password: &str, timeout_secs: u64) -> Result<String, Box<dyn std::error::Error>> {
    // 1. DO NOT set the clipboard here! The main process exits too fast.
    // We pass the password to the daemon and let IT hold the clipboard.

    let current_exe = env::current_exe()?;

    let mut cmd = Command::new(current_exe);
    cmd.env("VAULT_CLIPBOARD_DAEMON", "1")
       .env("VAULT_TARGET_PW", password)
       .env("VAULT_TIMEOUT", timeout_secs.to_string())
       .stdin(Stdio::null())
       .stdout(Stdio::null())
       .stderr(Stdio::null());

    #[cfg(unix)]
    {
        // Detach process completely from the terminal session on Unix/Linux
        cmd.spawn()?;
    }

    #[cfg(windows)]
    {
        cmd.spawn()?;
    }

    Ok(format!("[+] Password copied! It will clear in {} seconds.", timeout_secs))
}

/// Place this helper right at the top of your `main()`. 
pub fn init_clipboard_daemon() {
    if env::var("VAULT_CLIPBOARD_DAEMON").is_ok() {
        if let Ok(password) = env::var("VAULT_TARGET_PW") {
            let timeout: u64 = env::var("VAULT_TIMEOUT")
                .unwrap_or_else(|_| "15".to_string())
                .parse()
                .unwrap_or(15);

            // 1. The daemon takes control of the clipboard
            if let Ok(mut cb) = Clipboard::new() {
                let _ = cb.set_text(&password);

                // 2. The daemon stays alive, keeping the X11 clipboard request server active
                thread::sleep(Duration::from_secs(timeout));

                // 3. Clear the clipboard if it still holds our password
                if let Ok(current) = cb.get_text() {
                    if current == password {
                        let _ = cb.set_text("");
                    }
                }
            }
        }
        // Terminate the background daemon process instantly
        std::process::exit(0);
    }
}