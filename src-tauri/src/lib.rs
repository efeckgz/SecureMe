use std::fs::{create_dir_all, File};
use std::io::Write;
use tauri::Manager;

mod meta;
mod vault;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            vault::create_secure_vault,
            vault::get_vaults
        ])
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
                    if !dir.exists() {
                        match File::create(&dir) {
                            Ok(mut file) => {
                                let meta = meta::Meta::empty();
                                let json_str = serde_json::to_string(&meta)
                                    .expect("Could not conver empty meta object to json string.");
                                file.write_all(json_str.as_bytes())
                                    .expect("Could not write empty meta strign into file");
                            }
                            Err(e) => println!("Error creating the meta file: {}", e),
                        }
                    }
                }
            }
            Ok(())
        })
        .plugin(tauri_plugin_dialog::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
