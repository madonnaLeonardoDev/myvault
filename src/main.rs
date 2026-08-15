use crate::encrpyption::write::{derive_key, get_secure_input};

mod encrpyption;


fn main() {  
    let salt:[u8;16] = [8,123, 56, 5, 23, 66, 24 , 79 , 23 , 20, 129, 23, 245, 91, 70, 29];
    match get_secure_input("Insert Password: ") {
        Ok(password) => {
            println!("Password input succesful");
            match derive_key(&password, &salt) {
                Ok(key) => {
                    println!("Key: {:?}", key)
                },
                Err(_e) => {
                    print!("Error: {}", _e)
                }
            }
            //pasword logic here

        },
        Err(_e) => {
            println!("Couldnt get the password input")
        }
    }
}
