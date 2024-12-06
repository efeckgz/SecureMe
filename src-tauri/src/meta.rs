use serde::{Deserialize, Serialize};
use std::{
    fs::{self, File, OpenOptions},
    io::{self, Write},
    vec::Vec,
};
use tauri::Manager;

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

    pub fn empty() -> Self {
        Meta {
            paths: vec![],
            names: vec![],
            hashes: vec![],
        }
    }

    pub fn from_json(handle: tauri::AppHandle) -> io::Result<Self> {
        let mut data_dir = handle
            .path()
            .app_data_dir()
            .expect("The app data directory cannot be found.");
        data_dir.push("meta.json");

        let json_str = fs::read_to_string(&data_dir)
            .expect("Error reading json file contents from app data dir into json string");

        let result: Meta =
            serde_json::from_str(&json_str).expect("Failed parsing meta file as a Meta struct.");

        Ok(result)
    }

    pub fn to_json(&self, handle: tauri::AppHandle) -> io::Result<()> {
        let mut data_dir = handle
            .path()
            .app_data_dir()
            .expect("The app data directory cannot be found.");
        data_dir.push("meta.json");

        let json_str =
            serde_json::to_string(self).expect("Error converting data file back to json string.");

        let mut meta_file =
            File::create(data_dir).expect("Error opening meta file from the app data directory.");
        // let mut file = OpenOptions::new().write(true).create

        meta_file
            .write_all(json_str.as_bytes())
            .expect("Error writing json string to meta file.");

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
