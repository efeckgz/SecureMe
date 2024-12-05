use serde::{Deserialize, Serialize};
use std::{
    fs::{self, read_to_string, File},
    io::{self, Write},
    path::Path,
    vec::Vec,
};
use tauri::{
    path::{BaseDirectory, PathResolver},
    Manager,
};

#[derive(Serialize, Deserialize)]
pub struct Meta {
    pub paths: Vec<String>,
    pub names: Vec<String>,
    pub hashes: Vec<String>,
}

impl Meta {
    fn new(paths: Vec<String>, names: Vec<String>, hashes: Vec<String>) -> Self {
        Meta {
            paths,
            names,
            hashes,
        }
    }

    pub fn from_json(handle: tauri::AppHandle) -> io::Result<Self> {
        let data_dir = handle
            .path()
            .app_data_dir()
            .expect("The app data directory cannot be found.");
        let json_str = fs::read_to_string(&data_dir)?;
        let result: Meta = serde_json::from_str(&json_str)?;
        Ok(result)
    }

    pub fn to_json(&self, handle: tauri::AppHandle) -> io::Result<()> {
        let data_dir = handle
            .path()
            .app_data_dir()
            .expect("The app data directory cannot be found.");
        let json_str = serde_json::to_string(self)?;
        let mut meta_file = File::open(data_dir)?;
        meta_file.write_all(json_str.as_bytes())?;
        Ok(())
    }

    // pub fn append_to_meta(path: String, name: String, hash: String, handle: tauri::AppHandle) {
    //     if let Ok(mut meta) = Meta::from_json(handle) {
    //         meta.names.push(name);
    //         meta.hashes.push(hash);
    //         meta.paths.push(path);
    //     }
    // }
}
