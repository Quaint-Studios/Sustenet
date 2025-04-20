use sustenet_shared as shared;

use shared::security::aes::{ load_all_keys, KeyMap };

lazy_static::lazy_static! {
    pub static ref AES_KEYS: KeyMap = match load_all_keys() {
        Ok(keys) => keys,
        Err(e) => {
            println!("Failed to load keys: {:?}", e);
            KeyMap::new()
        }
    };
}

const PASSWORD_LEN: usize = 20;
pub fn generate_passphrase() -> [u8; PASSWORD_LEN] {
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                            abcdefghijklmnopqrstuvwxyz\
                            0123456789)(*&^%$#@!~";

    let mut password = [0u8; PASSWORD_LEN];
    getrandom::fill(&mut password).expect("Failed to generate password.");

    for byte in &mut password {
        *byte = CHARSET[(*byte as usize) % CHARSET.len()];
    }

    password
}
