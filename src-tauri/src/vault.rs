use serde::{Deserialize, Serialize};
use tauri_plugin_log::fern::meta;

use crate::meta::Meta;
use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::{self, SaltString};
use argon2::{Argon2, PasswordHasher};

#[derive(Serialize, Deserialize)]
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
    append_to_vaults(
        name.to_owned(),
        path.to_owned(),
        password.to_owned(),
        handle,
    );
    // let key = generate_key(salt, password)

    // Generate a key based on the password the user provided
    // Scan all the files in this directory
    // Interpret all files as byte arrays and encrypt them with the key.
    // concatanate all the cipeher-byte arrays into one cipher file - the vault
    // println!("Created vault: {}, {}, {}.", name, path, password);
    // println!("Password hash: {}", sha256::digest(password));
}

// Function to add the vault of the given properties into the metafile
fn append_to_vaults(name: String, path: String, password: String, handle: tauri::AppHandle) {
    // TODO: Implement checking for existing vaults
    match Meta::from_json(handle.clone()) {
        Ok(mut meta) => {
            let argon2 = Argon2::default();
            let salt = SaltString::generate(&mut OsRng); // generate a one time random salt
            let hash = argon2
                .hash_password(password.as_bytes(), &salt)
                .unwrap()
                .to_string();

            println!("Argon2 hash: {}", hash.clone());

            meta.append_new(path, name, hash, salt.as_str().to_string());
            meta.to_json(handle)
                .expect("Could not convert the updated meta file.");
        }
        Err(e) => println!("Error adding vault to meta file: {}", e),
    }
}

// fn generate_hash(argon2: &Argon2, password: String, salt: String) -> String {}
