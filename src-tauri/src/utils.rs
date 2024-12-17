use serde::{Deserialize, Serialize};
use std::io::Write;
use std::path::Path;

use std::fs::{self, DirEntry};

use crate::config::Config;

use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::SaltString;
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};

use aes_gcm::{
    aead::{Aead, AeadCore, KeyInit},
    Aes256Gcm, Key, Nonce,
};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VaultViewModel {
    name: String,
    path: String,
    is_locked: bool,
}

impl VaultViewModel {
    fn new(name: &String, path: &String, is_locked: bool) -> Self {
        VaultViewModel {
            name: name.to_string(),
            path: path.to_string(),
            is_locked,
        }
    }

    // Read the Configfile and return a vector of vaults to the frontend.
    // Call this from a command to make the handle valid
    pub fn get_from_file(handle: tauri::AppHandle) -> Vec<VaultViewModel> {
        let configfile = Config::from_json(handle).expect("Could not access configfile!");
        let mut result = vec![];
        for i in 0..configfile.hashes.len() {
            let vault = VaultViewModel::new(&configfile.names[i], &configfile.paths[i], true);
            result.push(vault);
        }
        result
    }
}

pub fn lock_vault(path: &str, key: &[u8]) -> Result<(), String> {
    let path = Path::new(path);

    let mut vaultfile_bytes: Vec<u8> = vec![];

    // Count the entries in the directory and place the file count at the start
    let entries: Vec<DirEntry> = path
        .read_dir()
        .expect("Could not read dirs")
        .filter_map(|entry| entry.ok())
        .filter(|entry| !is_dotfile(&entry))
        .collect();

    // The size of each file will take 8 bytes
    vaultfile_bytes.push((entries.len() * 8) as u8);

    // Place the sizes in order
    for entry in &entries {
        if is_dotfile(entry) {
            continue;
        }

        let metadata = fs::metadata(entry.path()).expect("Could not extract metadata from file!");
        println!("Size of {:?} is {} bytes.", entry.path(), metadata.len());
        let size_bytes = metadata.len().to_le_bytes();
        vaultfile_bytes.extend_from_slice(&size_bytes);
    }

    // Merge bytes of files into vaultfile_bytes
    for entry in entries {
        if is_dotfile(&entry) {
            continue;
        }

        println!("Appending file {:?}", entry.path());
        let file_bytes = fs::read(entry.path()).expect("Could not read file bytes!");
        vaultfile_bytes.extend_from_slice(&file_bytes);

        // Remove the plaintext file after adding its bytes to the vault.
        if let Err(e) = fs::remove_file(entry.path()) {
            return Err(format!("Error removing plaintext file: {}", e.to_string()));
        }
    }

    let ciphertext = encrypt_file(&vaultfile_bytes, key);
    let mut vaultfile = fs::File::create(format!("{}/vaultfile", path.to_str().unwrap())).unwrap();
    if let Err(e) = vaultfile.write_all(&ciphertext) {
        return Err(format!(
            "Error writing ciphertext bytes into vaultfile: {}",
            e
        ));
    }

    Ok(())
}

// Function to add the vault of the given properties into the Configfile
pub fn append_to_vaults(
    name: &str,
    path: &str,
    hash: &str,
    salt: SaltString,
    handle: tauri::AppHandle,
) {
    // TODO: Implement checking for existing vaults
    match Config::from_json(handle.clone()) {
        Ok(mut config) => {
            config.append_new(path, name, hash, salt.as_str(), true);
            config
                .to_json(handle)
                .expect("Could not convert the updated Config file.");
        }
        Err(e) => println!("Error adding vault to Config file: {}", e),
    }
}

// Generate hash and salt using Argon2
pub fn generate_hash_salt(argon2: &Argon2, password: &str) -> (String, SaltString) {
    let salt = SaltString::generate(&mut OsRng);
    let hash = argon2
        .hash_password(password.to_string().as_bytes(), &salt)
        .unwrap()
        .to_string();

    (hash, salt)
}

pub fn verify_password(argon2: &Argon2, hash: String, password: &str) -> bool {
    let parsed_hash =
        PasswordHash::new(&hash).expect("Could not parse password hash for verification.");
    argon2
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok()
}

// Use Argon2 to derive a key from hash and salt
pub fn derive_key<'a>(
    argon2: Argon2<'a>,
    password: &'a str,
    salt: &'a str,
    key_bytes: &'a mut [u8],
) {
    if let Err(e) = argon2.hash_password_into(password.as_bytes(), salt.as_bytes(), key_bytes) {
        panic!("Error deriving a key: {}", e);
    }
}

// Encrypt a file using the generated key. Use Aes256 with nonce.
pub fn encrypt_file(file: &[u8], key: &[u8]) -> Vec<u8> {
    let aes_key = Key::<Aes256Gcm>::from_slice(key);
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng);
    let cipher = Aes256Gcm::new(aes_key);

    // The encrypted file
    let cipertext = cipher
        .encrypt(&nonce, file)
        .expect("Error encrypting file!");

    let mut out: Vec<u8> = nonce.to_vec();
    out.extend_from_slice(&cipertext);

    out
}

// Decrypt a file using the generated key
pub fn decrypt_file(file: Vec<u8>, key: &[u8]) -> Vec<u8> {
    let aes_key = Key::<Aes256Gcm>::from_slice(key);
    let cipher = Aes256Gcm::new(aes_key);

    let (nonce_bytes, ciphertext) = file.split_at(12); // First 12 bytes is the nonce
    let nonce = Nonce::from_slice(nonce_bytes);

    let plaintext = cipher
        .decrypt(nonce, ciphertext)
        .expect("Failed to decrypt the file!");
    plaintext
}

// Returns true if the name of the file starts with a dot.
fn is_dotfile(entry: &DirEntry) -> bool {
    entry.file_name().to_str().unwrap().starts_with(".")
}
