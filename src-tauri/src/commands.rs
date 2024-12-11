use std::{fs, path::Path};

use argon2::Argon2;

use crate::{
    meta::Meta,
    utils::{
        append_to_vaults, decrypt_file, derive_key, generate_hash_salt, lock_vault,
        verify_password, VaultViewModel,
    },
};

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

    lock_vault(path, &key_bytes);

    append_to_vaults(name, path, &hash, salt, handle);

    // Generate a key based on the password the user provided
    // Scan all the files in this directory
    // Interpret all files as byte arrays and encrypt them with the key.
    // concatanate all the cipeher-byte arrays into one cipher file - the vault
    // println!("Created vault: {}, {}, {}.", name, path, password);
    // println!("Password hash: {}", sha256::digest(password));
}

#[tauri::command]
pub fn unlock_vault(path: &str, password: &str, handle: tauri::AppHandle) {
    let argon2 = Argon2::default();
    let metafile =
        Meta::from_json(handle).expect("Could not open the metafile for unlocking the vault!");
    let index = metafile.index_of_path(&path);

    let hash = metafile.get_hash(index);
    let salt = metafile.get_salt(index);

    // Fix the if here
    if !verify_password(&argon2, hash.to_string(), password) {
        println!("Password does not hash!");
    } else {
        println!("Password match!");
        let mut key_bytes = [0u8; 32];
        derive_key(argon2, password, salt, &mut key_bytes);

        let path = Path::new(path);
        for entry in path.read_dir().expect("Cannot read the cipher-dir!") {
            if let Ok(entry) = entry {
                let ciphertext =
                    fs::read(entry.path()).expect("Could not read ciphertext into a byte vector!");
                let plaintext = decrypt_file(ciphertext, &key_bytes);
                if let Err(e) = fs::write(entry.path(), &plaintext) {
                    println!("Could not write plaintext back to file: {}", e);
                }
            }
        }
    }
}
