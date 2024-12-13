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

    // Decrypt the vault - old
    // let path_p = Path::new(path);
    // for entry in path_p.read_dir().expect("Cannot read the cipher-dir!") {
    //     if let Ok(entry) = entry {
    //         let ciphertext =
    //             fs::read(entry.path()).expect("Could not read ciphertext into a byte vector!");
    //         let plaintext = decrypt_file(ciphertext, &key_bytes);
    //         if let Err(e) = fs::write(entry.path(), &plaintext) {
    //             println!("Could not write plaintext back to file: {}", e);
    //         }
    //     }
    // }

    // Decypt the vault - new
    let path_p = path::Path::new(path);
    let vaultfile_bytes = match fs::read(format!("{}/vaultfile", path_p.to_str().unwrap())) {
        Ok(vaultfile_bytes) => vaultfile_bytes,
        Err(e) => return Err(e.to_string()),
    };

    let plaintext_bytes = decrypt_file(vaultfile_bytes, &key_bytes);

    // Reconstruct the files
    let file_count = plaintext_bytes[0] as usize;
    let (sizes, data) = plaintext_bytes.split_at((file_count * 8) + 1);

    let mut sizeptr = 0;
    let mut dataptr = 0;
    for i in 0..file_count {
        let sizearr: [u8; 8] = sizes[sizeptr..sizeptr + 8].try_into().unwrap(); // 8 bytes make the size of this item
        let size = u64::from_le_bytes(sizearr) as usize; // Size of this item. Read this many bytes to reconstruct the file.

        let mut file_bytes = vec![];
        file_bytes.extend_from_slice(&data[dataptr..dataptr + size]);
        let mut file = fs::File::create(format!("{}/open/{}.png", path_p.to_str().unwrap(), i + 1))
            .expect("Could not create file for plaintext!");
        if let Err(e) = file.write_all(&file_bytes) {
            return Err(format!("Error writing plaintext bytes to file: {}", e));
        }

        dataptr += size; // The start of the next item is this item + its size
        sizeptr += 8; // Next size starts from 8 bytes later
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
