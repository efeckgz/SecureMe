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
                    println!(
                        "App data dir from rust: {}",
                        dir.as_path().to_str().unwrap()
                    );

                    if !dir.exists() {
                        if let Err(e) = create_dir_all(&dir) {
                            println!("Error creating app data directory!");
                            // return Err(e);
                        }
                    }

                    dir.push("meta.json");
                    if let Err(e) = File::create(dir) {
                        println!("Error creating the meta file!");
                    }

                    // if dir.exists() {
                    //     println!("The App data dir exists!");
                    // } else {
                    //     println!("The app data dir DOES NOT EXIST");
                    // }

                    // dir.push("meta.json");
                    // if !dir.as_path().exists() {
                    //     println!("Creating the meta file!");
                    //     if let Err(e) = File::create(dir) {
                    //         println!(
                    //             "Failed to create meta file in the app data directory: {}",
                    //             e
                    //         );
                    //     }
                    // }
                }
            }
            Ok(())
        })
        .plugin(tauri_plugin_dialog::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
