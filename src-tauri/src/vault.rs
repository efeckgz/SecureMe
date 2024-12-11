use aes_gcm::{aes, Error};
use serde::{Deserialize, Serialize};

use crate::meta::Meta;

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

    // Read the metafile and return a vector of vaults to the frontend.
    // Call this from a command to make the handle valid
    fn get_from_file(handle: tauri::AppHandle) -> Vec<VaultViewModel> {
        let metafile = Meta::from_json(handle).expect("Could not access metafile!");
        let mut result = vec![];
        for i in 0..metafile.hashes.len() {
            let vault = VaultViewModel::new(&metafile.names[i], &metafile.paths[i], true);
            result.push(vault);
        }
        result
    }
}

#[tauri::command]
pub fn get_vaults(handle: tauri::AppHandle) -> Vec<VaultViewModel> {
    VaultViewModel::get_from_file(handle)
}

#[tauri::command]
pub fn remove_vault(path: String, handle: tauri::AppHandle) {
    let mut metafile = Meta::from_json(handle.clone())
        .expect("Could not access the metafile for deleting a vault.");
    let index = metafile.paths.iter().position(|p| *p == path).unwrap(); // The index of the item to remove

    // Remove the items
    metafile.remove_index(index);

    metafile
        .to_json(handle)
        .expect("Could not conver the metafile back to json after deletion.");
}

#[tauri::command]
pub fn create_secure_vault(name: &str, path: &str, password: &str, handle: tauri::AppHandle) {
    let argon2 = Argon2::default();
    let (hash, salt) = generate_hash_salt(&argon2, password);

    let mut key_bytes = [0_u8; 32];
    derive_key(argon2, password, salt.as_str(), &mut key_bytes);

    let _ = key_bytes
        .to_ascii_lowercase()
        .iter()
        .map(|&c| println!("{}", c as char));

    // println!(
    //     "The key to the kingdom is... {}!",
    //     // std::str::from_utf8(&key_bytes).unwrap(),
    //     // key_bytes.to_ascii_lowercase().iter().map(|&c| println!("{}", *c as char));
    // );

    append_to_vaults(name, path, &hash, salt, handle);
    // let key = generate_key(salt, password)

    // Generate a key based on the password the user provided
    // Scan all the files in this directory
    // Interpret all files as byte arrays and encrypt them with the key.
    // concatanate all the cipeher-byte arrays into one cipher file - the vault
    // println!("Created vault: {}, {}, {}.", name, path, password);
    // println!("Password hash: {}", sha256::digest(password));
}

#[tauri::command]
pub fn unlock_vault(path: String, password: String, handle: tauri::AppHandle) {
    let argon2 = Argon2::default();
    let metafile =
        Meta::from_json(handle).expect("Could not open the metafile for unlocking the vault!");
    let index = metafile.index_of_path(&path);
    let hash = metafile.get_hash(index);

    if !verify_password(&argon2, hash.to_string(), password.as_str()) {
        println!("Password does not hash!");
    }

    println!("Password match!");
}

// Function to add the vault of the given properties into the metafile
fn append_to_vaults(
    name: &str,
    path: &str,
    hash: &str,
    salt: SaltString,
    handle: tauri::AppHandle,
) {
    // TODO: Implement checking for existing vaults
    match Meta::from_json(handle.clone()) {
        Ok(mut meta) => {
            meta.append_new(path, name, hash, salt.as_str());
            meta.to_json(handle)
                .expect("Could not convert the updated meta file.");
        }
        Err(e) => println!("Error adding vault to meta file: {}", e),
    }
}

// Generate hash and salt using Argon2
fn generate_hash_salt(argon2: &Argon2, password: &str) -> (String, SaltString) {
    let salt = SaltString::generate(&mut OsRng);
    let hash = argon2
        .hash_password(password.to_string().as_bytes(), &salt)
        .unwrap()
        .to_string();

    (hash, salt)
}

fn verify_password(argon2: &Argon2, hash: String, password: &str) -> bool {
    let parsed_hash =
        PasswordHash::new(&hash).expect("Could not parse password hash for verification.");
    argon2
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok()
}

// Use Argon2 to derive a key from hash and salt
fn derive_key<'a>(argon2: Argon2<'a>, password: &'a str, salt: &'a str, key_bytes: &'a mut [u8]) {
    if let Err(e) = argon2.hash_password_into(password.as_bytes(), salt.as_bytes(), key_bytes) {
        panic!("Error deriving a key: {}", e);
    }
}

// Encrypt a file using the generated key. Use Aes256 with nonce.
fn encrypt_file(file: &[u8], key: &[u8]) -> Vec<u8> {
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
fn decrypt_file(file: Vec<u8>, key: &[u8]) -> Vec<u8> {
    let aes_key = Key::<Aes256Gcm>::from_slice(key);
    let cipher = Aes256Gcm::new(aes_key);

    let (nonce_bytes, ciphertext) = file.split_at(12); // First 12 bytes is the nonce
    let nonce = Nonce::from_slice(nonce_bytes);

    let plaintext = cipher
        .decrypt(nonce, ciphertext)
        .expect("Failed to decrypt the file!");
    plaintext
}
