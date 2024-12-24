use hex;
use std::fs::{create_dir_all, File};
use std::io::Write;
use tauri::Manager;

mod commands;
mod config;
mod utils;
mod viewmodel;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            commands::create_secure_vault,
            commands::lock_vault,
            commands::unlock_vault,
            commands::get_vaults,
            commands::remove_vault,
        ])
        .setup(|app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }

            create_configfile(app);
            Ok(())
        })
        .plugin(tauri_plugin_dialog::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

// Create the config.json file in app data directory if it doesnt exist.
fn create_configfile(app: &mut tauri::App) {
    let path = app.path();
    if let Ok(mut dir) = path.app_data_dir() {
        if !dir.exists() {
            if let Err(e) = create_dir_all(&dir) {
                println!("Error creating app data directory: {}", e);
                // return e;
            }
        }

        dir.push("config");
        if !dir.exists() {
            match File::create(&dir) {
                Ok(mut file) => {
                    let config = config::Config::default();
                    let json_str = serde_json::to_string(&config)
                        .expect("Could not conver empty config object to json string.");
                    let encoded = hex::encode(json_str);

                    file.write_all(encoded.as_bytes())
                        .expect("Could not write empty config strign into file");
                }
                Err(e) => println!("Error creating the config file: {}", e),
            }
        }
    }
}
