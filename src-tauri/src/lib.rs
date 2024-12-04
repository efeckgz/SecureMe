use std::fs::File;
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

                    dir.push("meta.json");
                    if !dir.exists() {
                        println!("Creating the meta file!");
                        File::create(dir);
                    }
                }
            }
            Ok(())
        })
        .plugin(tauri_plugin_dialog::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
