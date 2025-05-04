use std::sync::LazyLock;

use sustenet_shared::security::aes::{ load_all_keys, KeyMap };

pub static AES_KEYS: LazyLock<KeyMap> = LazyLock::new(|| match load_all_keys() {
    Ok(keys) => keys,
    Err(e) => {
        println!("Failed to load keys: {:?}", e);
        KeyMap::new()
    }
});

const PASSWORD_LEN: usize = 20;
pub fn generate_passphrase() -> Result<[u8; PASSWORD_LEN], getrandom::Error> {
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                            abcdefghijklmnopqrstuvwxyz\
                            0123456789)(*&^%$#@!~";

    let mut password = [0u8; PASSWORD_LEN];
    getrandom::fill(&mut password)?;

    for byte in &mut password {
        *byte = CHARSET[(*byte as usize) % CHARSET.len()];
    }

    Ok(password)
}
