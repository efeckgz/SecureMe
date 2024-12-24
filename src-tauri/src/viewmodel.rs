use crate::config::Config;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VaultViewModel {
    name: String,
    path: String,
    is_locked: bool,
}

impl VaultViewModel {
    fn new(name: &String, path: &String, is_locked: bool) -> Self {
        VaultViewModel {
            name: name.to_string(),
            path: path.to_string(),
            is_locked,
        }
    }

    // Read the Configfile and return a vector of vaults to the frontend.
    // Call this from a command to make the handle valid
    pub fn get_from_file(handle: tauri::AppHandle) -> Vec<VaultViewModel> {
        let configfile = Config::from_json(handle).expect("Could not access configfile!");
        let mut result = vec![];
        for i in 0..configfile.hashes.len() {
            let vault = VaultViewModel::new(
                &configfile.names[i],
                &configfile.paths[i],
                configfile.is_locked[i],
            );
            result.push(vault);
        }
        result
    }
}
