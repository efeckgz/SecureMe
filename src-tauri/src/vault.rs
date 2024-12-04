#[tauri::command]
pub fn create_secure_vault(path: String, password: String) {
    // Generate a key based on the password the user provided
    // Scan all the files in this directory
    // Interpret all files as byte arrays and encrypt them with the key.
    // concatanate all the cipeher-byte arrays into one cipher file - the vault
    println!("Created vault: {}, {}.", path, password);
}
