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

    // The salts are held in the metafile. This is a security vulnerability.
    // In a real life scenario the salt should be stored more securely.
    pub salts: Vec<String>,
}

#[allow(dead_code)]
impl Meta {
    // Creates an empty meta object
    pub fn empty() -> Self {
        Meta {
            paths: vec![],
            names: vec![],
            hashes: vec![],
            salts: vec![],
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

    // Remove an entry at a given index
    pub fn remove_index(&mut self, index: usize) {
        self.paths.remove(index);
        self.names.remove(index);
        self.hashes.remove(index);
        self.salts.remove(index);
    }

    // Append new vault to the file based on the given parameters.
    pub fn append_new(&mut self, path: &str, name: &str, hash: &str, salt: &str) {
        self.paths.push(path.to_string());
        self.names.push(name.to_string());
        self.hashes.push(hash.to_string());
        self.salts.push(salt.to_string());
    }

    // Returns the index of the entry of the given path
    pub fn index_of_path(&self, path: &str) -> usize {
        let index = self
            .paths
            .iter()
            .position(|p| p == path)
            .expect("Could not find the specified path in metafile!");
        index
    }

    // Getters
    pub fn get_path(&self, index: usize) -> &str {
        self.paths
            .get(index)
            .expect("Could not retrieve path: index out of bounds!")
    }

    pub fn get_name(&self, index: usize) -> &str {
        self.names
            .get(index)
            .expect("Could not retrieve name: index out of bounds!")
    }

    pub fn get_hash(&self, index: usize) -> &str {
        self.hashes
            .get(index)
            .expect("Could not retrieve hash: index out of bounds!")
    }

    pub fn get_salt(&self, index: usize) -> &str {
        self.salts
            .get(index)
            .expect("Could not retrieve salt: index out of bounds!")
    }
}
