use std::{fs, io::Write, path};

use argon2::Argon2;

use crate::{
    config::Config,
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
    // println!("Created vault: {}, {}, {}.", name, path_p, password);
    // println!("Password hash: {}", sha256::digest(password));
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

    // Decypt the vault - new
    let path_p = path::Path::new(path);
    let vaultfile_bytes = match fs::read(format!("{}/vaultfile", path_p.to_str().unwrap())) {
        Ok(vaultfile_bytes) => vaultfile_bytes,
        Err(e) => return Err(e.to_string()),
    };

    let plaintext_bytes = decrypt_file(vaultfile_bytes, &key_bytes);

    // Reconstruct the files
    let file_count = plaintext_bytes[0] as usize;

    let sizes = plaintext_bytes[1..=file_count].to_vec();
    let data = plaintext_bytes[file_count + 1..].to_vec();

    let mut bytes_read: usize = 0;
    let mut data_read: usize = 0;
    while bytes_read < sizes.len() {
        let size_bytes = sizes[bytes_read..bytes_read + 8].try_into().unwrap();
        let size = u64::from_le_bytes(size_bytes) as usize;
        println!("File size when decrypting: {}", size);

        // Read size as many bytes into a vector
        let mut file_bytes: Vec<u8> = vec![];
        file_bytes.extend_from_slice(&data[data_read..data_read + size]);

        // Construct a file out of these bytes
        let mut file = fs::File::create(format!(
            "{}/{}.png",
            path_p.to_str().unwrap(),
            (bytes_read + 8) / 8
        ))
        .expect("Could not create a file for decrypted data!");
        if let Err(e) = file.write_all(&file_bytes) {
            return Err(format!(
                "Error writing plaintext bytes to new file: {}",
                e.to_string()
            ));
        }

        bytes_read += 8;
        data_read += size;
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
