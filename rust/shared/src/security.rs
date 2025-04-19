pub mod base64engine {
    use base64::{ alphabet, engine::{ self, general_purpose }, Engine };

    const CUSTOM_ENGINE: engine::GeneralPurpose = engine::GeneralPurpose::new(
        &alphabet::URL_SAFE,
        general_purpose::NO_PAD
    );
    pub fn base64_encode(input: &[u8]) -> String {
        CUSTOM_ENGINE.encode(input)
    }

    pub fn base64_decode(input: &str) -> Vec<u8> {
        CUSTOM_ENGINE.decode(input).unwrap()
    }
}

pub mod aes {
    use std::{ collections::HashMap, fs::File, io::{ Read, Write }, vec };

    use aes_gcm::{
        aead::{ Aead, AeadCore, KeyInit, OsRng },
        Aes256Gcm, // Or `Aes128Gcm`
        Key,
        Nonce,
    };

    pub fn generate_key() -> Key<Aes256Gcm> {
        Aes256Gcm::generate_key(OsRng)
    }

    pub fn save_key(name: &str, key: Key<Aes256Gcm>) -> std::io::Result<()> {
        let mut file = File::create(format!("keys/{name}"))?;
        file.write_all(key.as_slice())?;
        Ok(())
    }

    pub fn load_key(name: &str) -> std::io::Result<Key<Aes256Gcm>> {
        let mut file = match File::open(format!("keys/{name}")) {
            Ok(file) => file,
            Err(_) => {
                return Err(
                    std::io::Error::new(std::io::ErrorKind::NotFound, "Failed to open file.")
                );
            }
        };
        let mut buf = vec![];
        match file.read_to_end(&mut buf) {
            Ok(_) => {
                if buf.is_empty() {
                    return Err(
                        std::io::Error::new(std::io::ErrorKind::InvalidData, "Key is empty.")
                    );
                }

                if buf.len() != 32 {
                    return Err(
                        std::io::Error::new(std::io::ErrorKind::InvalidData, "Key is not 32 bytes.")
                    );
                }
            }
            Err(_) => {
                return Err(std::io::Error::new(std::io::ErrorKind::Other, "Failed to read file."));
            }
        }
        Ok(Key::<Aes256Gcm>::from_slice(buf.as_slice()).to_owned())
    }

    pub type KeyMap = HashMap<String, Key<Aes256Gcm>>;

    pub fn load_all_keys() -> std::io::Result<HashMap<String, Key<Aes256Gcm>>> {
        let mut keys = HashMap::new();

        let entries = match std::fs::read_dir("keys") {
            Ok(entries) => entries,
            Err(_) => {
                return Err(
                    std::io::Error::new(std::io::ErrorKind::NotFound, "Directory 'keys' missing.")
                );
            }
        };

        for entry in entries {
            let entry = match entry {
                Ok(entry) => entry,
                Err(_) => {
                    continue;
                }
            };
            let name = match entry.file_name().to_str() {
                Some(name) => name.to_string(),
                None => {
                    continue;
                }
            };
            let key = match load_key(name.as_str()) {
                Ok(key) => key,
                Err(_) => {
                    continue;
                }
            };
            keys.insert(name, key);
        }
        Ok(keys)
    }

    pub fn encrypt(data: &[u8], key: &Key<Aes256Gcm>) -> Vec<u8> {
        let nonce = Aes256Gcm::generate_nonce(&mut OsRng); // 96-bits; unique per message
        let cipher = Aes256Gcm::new(&key);

        let ciphered_data = cipher.encrypt(&nonce, data).expect("Failed to encrypt data.");
        [nonce.as_slice(), ciphered_data.as_slice()].concat()
    }

    pub fn decrypt(data: &[u8], key: &Key<Aes256Gcm>) -> Vec<u8> {
        let (nonce, data) = data.split_at(12);
        let nonce = Nonce::from_slice(nonce);
        let cipher = Aes256Gcm::new(&key);
        cipher.decrypt(nonce, data).expect("Failed to decrypt data. Maybe the key doesn't match the name?")
    }
}

#[cfg(test)]
pub mod tests {
    use super::{ aes::*, base64engine::* };

    #[test]
    pub fn test_key_gen_encode_and_decode() {
        let key = generate_key();
        let keyb64 = base64_encode(key.as_slice());
        let key2 = base64_decode(&keyb64);
        assert_ne!(key.as_slice(), keyb64.as_bytes());
        assert_eq!(key.as_slice(), key2)
    }

    #[test]
    pub fn test_encrypt_and_decrypt() {
        let key = generate_key();
        let data = b"Hello, World!";
        let encrypted_data = encrypt(data, &key);
        let decrypted_data = decrypt(encrypted_data.as_slice(), &key);
        assert_ne!(data, encrypted_data.as_slice());
        assert_eq!(data, decrypted_data.as_slice());
    }

    #[test]
    pub fn test_save_key() {
        match save_key("cluster_key2", generate_key()) {
            Ok(_) => {}
            Err(e) => {
                println!("Failed to save key: {:?}", e);
            }
        };
    }

    #[test]
    pub fn test_save_key_and_load_key() {
        let key = generate_key();

        match save_key("../cluster_key3", key) {
            Ok(_) => {}
            Err(e) => {
                println!("Failed to save key: {:?}", e);
                return;
            }
        }

        let key2 = match load_key("cluster_key3") {
            Ok(key) => key,
            Err(e) => {
                println!("Failed to load key: {:?}", e);
                return;
            }
        };

        assert_eq!(key.as_slice(), key2.as_slice());
    }

    #[test]
    pub fn test_load_key() {
        println!("Dir: {:?}", std::fs::read_dir("keys"));
        let key = match load_key("cluster_key") {
            Ok(key) => key,
            Err(e) => {
                println!("Failed to load key: {:?}", e);
                return;
            }
        };
        assert_eq!(key.as_slice().len(), 32);
    }

    #[test]
    pub fn test_load_all_keys() {
        let keys = match load_all_keys() {
            Ok(keys) => keys,
            Err(e) => {
                println!("Failed to load all keys: {:?}", e);
                return;
            }
        };
        assert_eq!(keys.len(), 2);
    }

    #[test]
    pub fn test_path() {
        println!("Path: {:?}", workspace_dir());
    }

    fn workspace_dir() -> std::path::PathBuf {
        let output = std::process::Command::new(env!("CARGO"))
            .arg("locate-project")
            .arg("--workspace")
            .arg("--message-format=plain")
            .output()
            .unwrap()
            .stdout;
        let cargo_path = std::path::Path::new(std::str::from_utf8(&output).unwrap().trim());
        cargo_path.parent().unwrap().to_path_buf()
    }
}
