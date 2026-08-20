use std::{ io::{self, Read, Write}, vec};
use serde::{Deserialize, Serialize};
use zeroize::{ Zeroizing};
use myvault::{config::Settings,  read_write::{PasswordBlock, UnlockedVault}};

#[derive(Serialize, Deserialize)]
struct InMsg {
    action: String,
    website: Zeroizing<String>, //known as service in PasswordBlock
    username: Zeroizing<String>,
    password: Zeroizing<String>
}

#[derive(Serialize, Deserialize)]
struct OutMsg {
    status: String,
    message: String
}

#[derive(Serialize, Deserialize)]
struct PasswordMatches {
    username: Zeroizing<String>,
    password: Zeroizing<String>
}

fn main() {
    let mut loaded_services: Vec<Zeroizing<String>> = Vec::new(); 

    let mut loaded_pw: Option<Zeroizing<String>> = Option::None;

    fn check_loaded_pw (pw: &Option<Zeroizing<String>>) -> Result<Zeroizing<String>, ()> {
        if pw.is_none() {
        write_packet(OutMsg { status: "request_password".to_string(), message: "Insert Password".to_string()});
            let pw = match read_packet() {
                Ok(v) => {
                    if v.password.is_empty()
                        {
                            return Err(());
                        }
                                v.password
                    },
                    Err(_) => return Err(())
            };
            return Ok(pw);
        };
        return Ok(pw.clone().unwrap());
    }

    loop {
        let incoming_packet: InMsg = match read_packet() {
            Ok(packet) => packet,
            Err(_) => {break;}
        };

        match incoming_packet.action.as_str() {
            "init_session" | "init_session_pw_remeber" => {
                if incoming_packet.password.is_empty() {
                    break;
                }
                if incoming_packet.action == "init_session_pw_remeber" {
                   loaded_pw = Some(incoming_packet.password.clone());
                }
                let settings = match Settings::load() {
                    Ok(s) => s.0,
                    Err(_) => break
                };
                let unlocked_vault = match UnlockedVault::open(&incoming_packet.password, &settings) {
                    Ok(v) => v,
                    Err(_) => break
                };
                for e in &unlocked_vault.data.vec_passwords_blocks {
                    loaded_services.push(e.service.clone());
                }
            },
            "opened_website_field" => {
                if incoming_packet.website.is_empty() {
                    break;
                }

                for e in &loaded_services {
                    if e == &incoming_packet.website {
                    let pw =match check_loaded_pw(&loaded_pw){
                    Ok(v) => v,
                    Err(()) => break
                        };

                        let settings = match Settings::load() {Ok(s)=> s, Err(_) => break}.0;
                        
                        let unlocked_vault = match UnlockedVault::open(&pw, &settings){
                            Ok(v) => v,
                            Err(_) => break
                        };
                        let mut found_matches: Vec<PasswordMatches> = Vec::new();
                        for e in &unlocked_vault.data.vec_passwords_blocks {
                            if &e.service == &incoming_packet.website {
                                found_matches.push(PasswordMatches{username: e.username.clone(), password: e.password.clone()});
                            }
                        }
                        let found_matchs_str = match serde_json::to_string(&found_matches){
                            Ok(v) => v,
                            Err(_) => break
                        };


                            write_packet(OutMsg { status: "match_found".to_string(), message:  found_matchs_str});
            
                    }
                }


            },
            
            "submit_pw" => {
                if incoming_packet.website.is_empty()
                || incoming_packet.username.is_empty()
                || incoming_packet.password.is_empty() {
                    break;
                }
                let mut found = false;
                for e in &loaded_services {
                    if e == &incoming_packet.website{
                        found = true;
                        break;
                    }
                }
                if found{
                    continue;
                }
                let settings = match Settings::load() {
                    Ok(s) => s.0,
                    Err(_) => break
                };
                let pw =match check_loaded_pw(&loaded_pw){
                    Ok(v) => v,
                    Err(()) => break
                };
                let mut unlocked_vault = match UnlockedVault::open(&pw, &settings) {
                    Ok(v) => v,
                    Err(_) => break
                };
                unlocked_vault.add_password(PasswordBlock { service: incoming_packet.website.clone(), username: incoming_packet.username, password: incoming_packet.password });
                match unlocked_vault.save() {
                    Ok(()) => {
                        loaded_services.push(incoming_packet.website);
                        write_packet(OutMsg { status: "ok".to_string(), message: "Password saved to vault".to_string() });
                    },
                    Err(_) => break
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