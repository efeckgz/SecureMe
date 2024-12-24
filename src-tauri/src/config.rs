use serde::{Deserialize, Serialize};
use std::{
    fs::{self, File},
    io::{self, Write},
    vec::Vec,
};
use tauri::Manager;

#[derive(Serialize, Deserialize, Default)]
pub struct Config {
    pub paths: Vec<String>,
    pub names: Vec<String>,
    pub hashes: Vec<String>,

    // The salts are held in the configfile. This is a security vulnerability.
    // In a real life scenario the salt should be stored more securely.
    pub salts: Vec<String>,
    pub is_locked: Vec<bool>,
}

impl Config {
    // Parses the configfile into Config object
    pub fn from_json(handle: tauri::AppHandle) -> io::Result<Self> {
        let mut data_dir = handle
            .path()
            .app_data_dir()
            .expect("The app data directory cannot be found.");
        data_dir.push("config");

        let encoded = fs::read_to_string(&data_dir)
            .expect("Error reading json file contents from app data dir into json string");
        let json_str = String::from_utf8(hex::decode(encoded).unwrap()).unwrap();

        let result: Config =
            serde_json::from_str(&json_str).expect("Failed parsing configfile as a Config struct.");

        Ok(result)
    }

    // Converts from a Config object into into json file
    pub fn to_json(&self, handle: tauri::AppHandle) -> io::Result<()> {
        let mut data_dir = handle
            .path()
            .app_data_dir()
            .expect("The app data directory cannot be found.");
        data_dir.push("config");

        let json_str =
            serde_json::to_string(self).expect("Error converting data file back to json string.");
        let encoded = hex::encode(json_str);

        let mut configfile =
            File::create(data_dir).expect("Error opening meta file from the app data directory.");

        configfile
            .write_all(encoded.as_bytes())
            .expect("Error writing json string to meta file.");

        Ok(())
    }

    // Remove an entry at a given index
    pub fn remove_index(&mut self, index: usize) {
        self.paths.remove(index);
        self.names.remove(index);
        self.hashes.remove(index);
        self.salts.remove(index);
        self.is_locked.remove(index);
    }

    // Append new vault to the file based on the given parameters.
    pub fn append_new(&mut self, path: &str, name: &str, hash: &str, salt: &str, is_locked: bool) {
        self.paths.push(path.to_string());
        self.names.push(name.to_string());
        self.hashes.push(hash.to_string());
        self.salts.push(salt.to_string());
        self.is_locked.push(is_locked);
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

    // Returns true if a path is in the config.
    pub fn path_exists(&self, path: &str) -> bool {
        self.paths.contains(&path.to_string())
    }

    pub fn index_locked(&self, index: usize) -> bool {
        self.is_locked[index]
    }

    pub fn mark_unlocked(&mut self, index: usize) {
        self.is_locked[index] = false;
    }

    pub fn mark_locked(&mut self, index: usize) {
        self.is_locked[index] = true;
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
