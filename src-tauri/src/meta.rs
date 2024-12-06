use serde::{Deserialize, Serialize};
use std::{
    fs::{self, File},
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
    // Creates an empty meta object
    pub fn empty() -> Self {
        Meta {
            paths: vec![],
            names: vec![],
            hashes: vec![],
        }
    }

    // Parses the metafile into Meta object
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

    // Converts from a Meta object into metafile
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

        meta_file
            .write_all(json_str.as_bytes())
            .expect("Error writing json string to meta file.");

        Ok(())
    }
}
