use std::fs::{self, DirEntry};
use std::io::Write;
use std::path::Path;

use rand::prelude::*;
use rand::rngs::StdRng;

use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::SaltString;
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};

use aes_gcm::{
    aead::{Aead, AeadCore, KeyInit},
    Aes256Gcm, Key, Nonce,
};

use crate::config::Config;

// Utility function to lock a vault.
pub fn lock_vault_util(path: &str, key: &[u8]) -> Result<(), String> {
    let path = Path::new(path);

    // Count the entries in the directory and place the file count at the start
    let entries: Vec<DirEntry> = path
        .read_dir()
        .expect("Could not read dirs")
        .filter_map(|entry| entry.ok())
        .filter(|entry| !is_dotfile(&entry))
        .collect();

    let seed = calculate_seed(path.to_str().unwrap());
    let vaultfile_bytes = create_vaultfile_bytes(&entries, seed)?;
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

// Reconstruct the files of the directory from the decrypted vault bytes.
pub fn reconstruct_files(
    mut plaintext_bytes: Vec<u8>,
    path: &std::path::Path,
) -> Result<(), String> {
    // Shuffle back
    let seed = calculate_seed(path.to_str().unwrap());
    // let mut rng = StdRng::seed_from_u64(seed);
    // plaintext_bytes.shuffle(&mut rng);
    unshuffle_bytes(&mut plaintext_bytes, seed);

    let file_count = plaintext_bytes[0] as usize;

    let sizes = plaintext_bytes[1..=file_count].to_vec();
    let data = plaintext_bytes[file_count + 1..].to_vec();

    let mut bytes_read: usize = 0;
    let mut data_read: usize = 0;
    while bytes_read < sizes.len() {
        let size_bytes = sizes[bytes_read..bytes_read + 8].try_into().unwrap();
        let size = u64::from_le_bytes(size_bytes) as usize;

        // Read size as many bytes into a vector
        let mut file_bytes: Vec<u8> = vec![];
        file_bytes.extend_from_slice(&data[data_read..data_read + size]);

        // file_bytes now looks like
        // name_size b1 b2 b3 b4 b5 contents
        let (name_bytes, data_bytes) = file_bytes.split_at((file_bytes[0] + 1) as usize);
        let name = String::from_utf8(name_bytes.to_vec())
            .expect("Could not convert from name bytes into utf8 string.");

        // Construct a file out of these bytes
        let mut file = fs::File::create(format!("{}/{}", path.to_str().unwrap(), name))
            .expect("Could not create a file for decrypted data!");
        if let Err(e) = file.write_all(data_bytes) {
            return Err(format!(
                "Error writing plaintext bytes to new file: {}",
                e.to_string()
            ));
        }

        bytes_read += 8;
        data_read += size;
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

// Calculate a random seed for a vault path.
// Convert the path to a byte array, get first 8 and last 8 bytes as u64 and add them
// Take the power of 42.
// In the real world use something more secure.
fn calculate_seed(path: &str) -> u64 {
    let bytes = path.as_bytes().to_vec();
    let first_8 = u64::from_le_bytes(bytes[..8].try_into().unwrap());

    let start = if bytes.len() >= 8 { bytes.len() - 8 } else { 0 };
    let last_8 = u64::from_le_bytes(bytes[start..].try_into().unwrap());
    // let last_8 = u64::from_le_bytes(bytes[8..].try_into().unwrap());
    let sum = first_8.wrapping_add(last_8);

    let seed = sum.wrapping_pow(42);
    seed
}

// Shuffle the given bytes in place using the given seed
fn shuffle_bytes(bytes: &mut Vec<u8>, seed: u64) {
    let mut rng = StdRng::seed_from_u64(seed);
    let indices: Vec<usize> = (0..bytes.len()).collect();
    let mut shuffle_map: Vec<_> = indices.clone();
    shuffle_map.shuffle(&mut rng);

    let mut shuffled = bytes.clone();
    for (i, &idx) in shuffle_map.iter().enumerate() {
        shuffled[i] = bytes[idx];
    }

    *bytes = shuffled;
}

// Unshuffle the bytes in place using the given seed
fn unshuffle_bytes(bytes: &mut Vec<u8>, seed: u64) {
    let mut rng = StdRng::seed_from_u64(seed);
    let indices: Vec<usize> = (0..bytes.len()).collect();
    let mut shuffle_map: Vec<_> = indices;
    shuffle_map.shuffle(&mut rng);

    let mut unshuffled = bytes.clone();
    for (i, &idx) in shuffle_map.iter().enumerate() {
        unshuffled[idx] = bytes[i];
    }

    *bytes = unshuffled;
}

// Create bytes of vaultfile from a list of entries
fn create_vaultfile_bytes(entries: &Vec<DirEntry>, seed: u64) -> Result<Vec<u8>, String> {
    let mut vaultfile_bytes = vec![];

    // The size of each file will take 8 bytes
    vaultfile_bytes.push((entries.len() * 8) as u8);

    // Place the sizes in order
    for entry in entries {
        let metadata = fs::metadata(entry.path()).expect("Could not extract metadata from file!");
        // let size_bytes = metadata.len().to_le_bytes();

        // Size will be the file size + the size of the name + 1 more byte indicating the size of the name
        let file_size = metadata.len();
        let name_size = entry.file_name().to_str().unwrap().len() as u64;
        let total_size = file_size + name_size + 1;

        let size_bytes = total_size.to_le_bytes();
        vaultfile_bytes.extend_from_slice(&size_bytes);
    }

    // Merge bytes of files into vaultfile_bytes
    for entry in entries {
        let file_bytes = fs::read(entry.path()).expect("Could not read file bytes!");

        // Place the size of the name before the contents
        let name = entry.file_name();

        let name_size = name.to_str().unwrap().len() as u8;
        vaultfile_bytes.push(name_size);

        // Place the name bytes beofre the contents
        let name_bytes = name.to_str().unwrap().as_bytes();
        vaultfile_bytes.extend_from_slice(name_bytes);

        // Place the file contents
        vaultfile_bytes.extend_from_slice(&file_bytes);

        // Remove the plaintext file after adding its bytes to the vault.
        if let Err(e) = fs::remove_file(entry.path()) {
            return Err(format!("Error removing plaintext file: {}", e.to_string()));
        }
    }

    // Shuffle the bytes in plaintext for more security
    // let mut rng = StdRng::seed_from_u64(seed);
    // vaultfile_bytes.shuffle(&mut rng);
    shuffle_bytes(&mut vaultfile_bytes, seed);

    Ok(vaultfile_bytes)
}

// Returns true if the name of the file starts with a dot.
fn is_dotfile(entry: &DirEntry) -> bool {
    entry.file_name().to_str().unwrap().starts_with(".")
}
