use serde::{Deserialize, Serialize};
use std::{path::Path, vec::Vec};
use tauri::{
    path::{BaseDirectory, PathResolver},
    Manager,
};

#[derive(Serialize, Deserialize)]
pub struct Meta {
    names: Vec<String>,
    hashes: Vec<String>,
}

pub fn create_meta_file() {
    // let path = tauri::App::path(&self)
}
