use serde::{Deserialize, Serialize};

use crate::meta::Meta;

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
    metafile.names.remove(index);
    metafile.hashes.remove(index);
    metafile.paths.remove(index);

    metafile
        .to_json(handle)
        .expect("Could not conver the metafile back to json after deletion.");
}

#[tauri::command]
pub fn create_secure_vault(name: String, path: String, password: String, handle: tauri::AppHandle) {
    append_to_vaults(name, path, password, handle);

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
            let hash = sha256::digest(password);
            meta.paths.push(path);
            meta.names.push(name);
            meta.hashes.push(hash);
            meta.to_json(handle)
                .expect("Could not convert the updated meta file.");
        }
        Err(e) => println!("Error adding vault to meta file: {}", e),
    }
}
