use std::{fs, path};

use argon2::Argon2;

use crate::{
    config::Config,
    utils::{
        append_to_vaults, decrypt_file, derive_key, generate_hash_salt, lock_vault,
        reconstruct_files, verify_password, VaultViewModel,
    },
};

#[tauri::command]
pub fn get_vaults(handle: tauri::AppHandle) -> Vec<VaultViewModel> {
    VaultViewModel::get_from_file(handle)
}

#[tauri::command]
pub fn remove_vault(path: &str, handle: tauri::AppHandle) {
    let mut configfile = Config::from_json(handle.clone())
        .expect("Could not access the configfile for deleting a vault.");
    let index = configfile.paths.iter().position(|p| *p == path).unwrap(); // The index of the item to remove

    // Remove the items
    configfile.remove_index(index);

    configfile
        .to_json(handle)
        .expect("Could not conver the configfile back to json after deletion.");
}

#[tauri::command]
pub fn create_secure_vault(
    name: &str,
    path: &str,
    password: &str,
    handle: tauri::AppHandle,
) -> Result<(), String> {
    let argon2 = Argon2::default();
    let (hash, salt) = generate_hash_salt(&argon2, password);

    let mut key_bytes = [0_u8; 32];
    derive_key(argon2, password, salt.as_str(), &mut key_bytes);

    lock_vault(path, &key_bytes)?;

    append_to_vaults(name, path, &hash, salt, handle);

    Ok(())

    // Generate a key based on the password the user provided
    // Scan all the files in this directory
    // Interpret all files as byte arrays and encrypt them with the key.
    // concatanate all the cipeher-byte arrays into one cipher file - the vault
}

#[tauri::command]
pub fn unlock_vault(path: &str, password: &str, handle: tauri::AppHandle) -> Result<(), String> {
    // Read the config
    let mut configfile = Config::from_json(handle.clone())
        .expect("Could not open the configfile for unlocking the vault!");

    // Variables
    let argon2 = Argon2::default();
    let index = configfile.index_of_path(&path);
    let hash = configfile.get_hash(index);
    let salt = configfile.get_salt(index);

    // Verify password
    if !verify_password(&argon2, hash.to_string(), password) {
        return Err("Incorrect password!".to_string());
    }

    // Derive the key
    let mut key_bytes = [0u8; 32];
    derive_key(argon2, password, salt, &mut key_bytes);

    let path_p = path::Path::new(path);
    let vaultfile_bytes = match fs::read(format!("{}/vaultfile", path_p.to_str().unwrap())) {
        Ok(vaultfile_bytes) => vaultfile_bytes,
        Err(e) => return Err(e.to_string()),
    };

    let plaintext_bytes = decrypt_file(vaultfile_bytes, &key_bytes);
    reconstruct_files(plaintext_bytes, &path_p)?;

    // Remove the vaultfile after decryption
    if let Err(e) = fs::remove_file(format!("{}/vaultfile", path_p.to_str().unwrap())) {
        return Err(format!(
            "Error removing vaultfile after decryption: {}",
            e.to_string()
        ));
    }

    // Mark the path unlocked in and save the config
    configfile.mark_unlocked(index);
    if let Err(e) = configfile.to_json(handle) {
        return Err(format!(
            "Could not save updated config file into json: {}",
            e
        ));
    }

    Ok(())
}
