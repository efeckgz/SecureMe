use std::fs::{create_dir_all, File};
use std::io::Error;
use tauri::Manager;

mod meta;
mod vault;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![vault::create_secure_vault])
        .setup(|app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;

                // Create the meta.json file if not present
                let path = app.path();
                if let Ok(mut dir) = path.app_data_dir() {
                    if !dir.exists() {
                        if let Err(e) = create_dir_all(&dir) {
                            println!("Error creating app data directory: {}", e);
                            // return e;
                        }
                    }

                    dir.push("meta.json");
                    if let Err(e) = File::create(dir) {
                        println!("Error creating the meta file: {}", e);
                    }
                }
            }
            Ok(())
        })
        .plugin(tauri_plugin_dialog::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
