use crate::meta::Meta;

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
