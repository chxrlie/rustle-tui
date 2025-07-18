use std::collections::HashSet;
use std::fs::{self, File};
use std::io::{BufReader, BufWriter};
use std::path::PathBuf;
use bincode::{config, Decode, Encode};
use serde::{Deserialize, Serialize};

type PasswordHash = [u8; 32];

#[derive(Serialize, Deserialize, Encode, Decode)]
struct StoredHashes {
    hashes: HashSet<PasswordHash>,
}

pub struct StorageManager {
    storage_path: PathBuf,
    hashes: HashSet<PasswordHash>,
}

impl StorageManager {
    pub fn new() -> Self {
        let mut storage_path = dirs::config_dir().expect("Could not find config directory");
        storage_path.push("rustle-tui");
        fs::create_dir_all(&storage_path).expect("Could not create config directory");
        storage_path.push("history.bin");

        let hashes = if storage_path.exists() {
            let file = File::open(&storage_path).expect("Could not open history file");
            let mut reader = BufReader::new(file);
            let config = config::standard();
            let stored: StoredHashes = bincode::decode_from_std_read(&mut reader, config).unwrap_or_else(|_| StoredHashes { hashes: HashSet::new() });
            stored.hashes
        } else {
            HashSet::new()
        };

        Self { storage_path, hashes }
    }

    pub fn is_duplicate(&self, password: &str) -> bool {
        let hash = blake3::hash(password.as_bytes());
        self.hashes.contains(hash.as_bytes())
    }

    pub fn store_password(&mut self, password: &str) {
        let hash = blake3::hash(password.as_bytes());
        self.hashes.insert(*hash.as_bytes());
        
        let file = File::create(&self.storage_path).expect("Could not create history file");
        let mut writer = BufWriter::new(file);
        let stored = StoredHashes { hashes: self.hashes.clone() };
        let config = config::standard();
        bincode::encode_into_std_write(&stored, &mut writer, config).expect("Could not write to history file");
    }
}