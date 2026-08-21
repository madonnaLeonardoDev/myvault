use std::{ io::{self, Read, Write}, vec};
use serde::{Deserialize, Serialize};
use zeroize::{ Zeroizing};
use myvault::{config::Settings,  read_write::{PasswordBlock, UnlockedVault}};
use std::fs::OpenOptions;

#[derive(Serialize, Deserialize, Debug)]
struct InMsg {
    action: String,
    website: Zeroizing<String>, //known as service in PasswordBlock
    username: Zeroizing<String>,
    password: Zeroizing<String>
}

#[derive(Serialize, Deserialize)]
struct OutMsg {
    status: String,
    message: String,
}



fn main() {
    let mut loaded_services: Vec<Zeroizing<String>> = Vec::new(); 

    let mut pw_opt: Option<Zeroizing<String>> = None;

    let settings = match Settings::load(){
        Ok(s) => s,
        Err(_) => {
            //TODO: ERROR OUT MSG
            return;
            }
        }.0;

    fn load_services(unlocked_vault: &UnlockedVault, loaded_services: &mut Vec<Zeroizing<String>>) {
    for e in &unlocked_vault.data.vec_passwords_blocks {
        loaded_services.push(e.service.clone());
    }
}
    
    
    loop {
        let incoming_packet: InMsg = match read_packet() {
            Ok(packet) => packet,
            Err(_) => {continue;}
        };
        match incoming_packet.action.as_str() {
            "ping" => {
         let mut file =match OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(true)
        .open("/home/mfleo/ping.txt") {
            Ok(v) => v,
            Err(_) => continue
        };

        match file.write_all(b"ping"){
            Ok(()) => {},
            Err(_) => continue
        }
                
            },
            "ext_loaded" => {
                write_packet(OutMsg { status: "ask_pw".to_string(), message: "key to init session".to_string() });
            }
            "password" => {
                if incoming_packet.password.is_empty(){
                    write_packet(OutMsg { status: "ask_pw".to_string(), message: "Cannot input empty key".to_string() });
                    continue;
                }

                match UnlockedVault::open(&incoming_packet.password, &settings) {
                    Ok(v) => {
                        load_services(&v,&mut loaded_services);
                        pw_opt = Some(incoming_packet.password.clone());
                    },
                    Err(_) => {
                        write_packet(OutMsg { status: "ask_pw".to_string(), message: "wrong key".to_string() });
                        continue;
                    }
                };
            },
            "field_focused" => {
                if incoming_packet.website.is_empty() {
                    write_packet(OutMsg { status: "error".to_string(), message: "Need a website String".to_string() });
                    continue;
                }
                let pw = match pw_opt {
                    Some(_) => pw_opt.as_ref().unwrap(),
                    None => {
                        write_packet(OutMsg { status: "ask_pw".to_string(), message: "key to init session".to_string() });
                        continue;
                    }
                };
                let mut found_vec: Vec<String> = Vec::new();
                for e in &loaded_services {
                    if incoming_packet.website == *e {
                        found_vec.push(e.to_string().clone());
                    }
                }
                if found_vec.len() == 0 {
                    continue;
                }
                
                let unlocked_vault = match UnlockedVault::open(pw, &settings) {
                    Ok(v) => v,
                    Err(e) => {
                        write_packet(OutMsg { status: "error".to_string(), message: e.to_string() });
                        continue;
                    }
                };
                let mut match_pw_vec: Vec<&PasswordBlock> = Vec::new();
                for p in &unlocked_vault.data.vec_passwords_blocks {
                    if p.service == incoming_packet.website{
                        match_pw_vec.push(p);
                    }
                };
                let matches_str = match serde_json::to_string(&match_pw_vec){
                    Ok(v) => v,
                    Err(e) => {
                        write_packet(OutMsg { status: "error".to_string(), message: e.to_string() });
                        continue;
                    }
                };
                write_packet(OutMsg { status: "match_found".to_string(), message: matches_str });
            },
            "save_pw" => {
                if incoming_packet.website.is_empty() 
                || incoming_packet.username.is_empty()
                || incoming_packet.password.is_empty() {
                    //TODO: err out msg
                    continue;
                }

                let pw = match pw_opt {
                    Some(_) => pw_opt.as_ref().unwrap(),
                    None => {
                        write_packet(OutMsg { status: "ask_pw".to_string(), message: "key to init session".to_string()});
                        continue;
                    }
                };
                let mut unlocked_vault = match UnlockedVault::open(pw, &settings){
                    Ok(v) => v,
                    Err(e) => {
                        write_packet(OutMsg { status: "error".to_string(), message: e.to_string() });
                        continue;
                    }
                };
                let mut overwrite = false;
                for e in &unlocked_vault.data.vec_passwords_blocks{
                    if e.service == incoming_packet.website && e.username == incoming_packet.username {
                    write_packet(OutMsg { status: "overwrite_save".to_string(), message: format!("[\"{}\" ,\"{}\" ,\"{}\"]",&*incoming_packet.website, &*incoming_packet.username, &*incoming_packet.password) });
                    overwrite = true;
                    }
                }
                if overwrite {
                    continue;
                }
                unlocked_vault.add_password(PasswordBlock { service: incoming_packet.website, username: incoming_packet.username, password: incoming_packet.password });
                match unlocked_vault.save(){
                    Ok(_) => {
                        load_services(&unlocked_vault,&mut loaded_services);
                        write_packet(OutMsg { status: "ok".to_string(), message: "added password to vault".to_string() });
                    },
                    Err(e) => {
                        write_packet(OutMsg { status: "error".to_string(), message: e.to_string() });
                        continue;
                    }
                }
            },
            "overwrite_pw" => {
                if incoming_packet.website.is_empty() 
                || incoming_packet.username.is_empty()
                || incoming_packet.password.is_empty() {
                    //TODO: err out msg
                    continue;
                }

                let pw = match pw_opt {
                    Some(_) => pw_opt.as_ref().unwrap(),
                    None => {
                        write_packet(OutMsg { status: "ask_pw".to_string(), message: "key to init session".to_string()});
                        continue;
                    }
                };
                let mut unlocked_vault = match UnlockedVault::open(pw, &settings){
                    Ok(v) => v,
                    Err(e) => {
                        write_packet(OutMsg { status: "error".to_string(), message: e.to_string() });
                        continue;
                    }
                };
                match unlocked_vault.remove_password(&incoming_packet.website, &incoming_packet.username) {
                    Ok(_) => {},
                    Err(e) => {
                        write_packet(OutMsg { status: "error".to_string(), message: e.to_string() });
                        continue;
                    }
                }
                unlocked_vault.add_password(PasswordBlock { service: incoming_packet.website, username: incoming_packet.username, password: incoming_packet.password });
                match unlocked_vault.save(){
                    Ok(_) => {
                        load_services(&unlocked_vault,&mut loaded_services);
                        write_packet(OutMsg { status: "ok".to_string(), message: "added password to vault".to_string() });
                    },
                    Err(e) => {
                        write_packet(OutMsg { status: "error".to_string(), message: e.to_string() });
                        continue;
                    }
                }
            }
            _=> {}
        }
    }
}





fn read_packet() -> Result<InMsg, String> {
    let mut len_bytes = [0u8; 4];
    io::stdin().read_exact(&mut len_bytes)
        .map_err(|e| e.to_string())?;
    let length = u32::from_le_bytes(len_bytes) as usize;

    let mut packet_slice = vec![0u8;length];

    io::stdin().read_exact(&mut packet_slice)
        .map_err(|e|e.to_string())?;
    let toml_packet:InMsg = serde_json::from_slice(&packet_slice)
        .map_err(|_| "Could not serialize stdin to json")?;

    Ok(toml_packet)
}

fn write_packet(out_packet: OutMsg) {
    let packet_bytes = match serde_json::to_vec(&out_packet) {
        Ok(v) => v,
        Err(_) => {
            let fallback_json = Vec::from(r#"{status:"error",message:"Serialization of stdout failed"}"#.as_bytes());
            fallback_json
        }
    };

    let length = packet_bytes.len() as u32;
    let mut stdout = io::stdout();

    // 2. Write and return early if anything fails
    if stdout.write_all(&length.to_le_bytes()).is_err() { return; }
    if stdout.write_all(&packet_bytes).is_err() { return; }
    let _ = stdout.flush();
}