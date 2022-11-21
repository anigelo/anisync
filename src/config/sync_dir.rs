use std::error::Error;
use std::path::PathBuf;
use serde::{Serialize, Deserialize};
use relative_path::RelativePathBuf;
use crate::fs_store;
use crate::fs_store::FILE;
use super::get_data_folder;

#[derive(Deserialize, Serialize, Debug)]
pub struct SyncDir {
    pub remote: String,
    pub local: RelativePathBuf
}

impl SyncDir {
    pub fn remote_abs(&self, remote_root: &str) -> String {
        format!("{}{}", remote_root, &self.remote)
    }
    pub fn local_abs(&self, local_root: &PathBuf) -> PathBuf {
        self.local.to_path(local_root)
    }
}

pub fn get_sync_folders() -> Vec<SyncDir> {
    let content = fs_store::read_file(get_data_folder(), FILE::SYNCS);
    return serde_json::from_str(&content.unwrap_or("[]".to_string())).unwrap();
}

pub fn update_sync_folders(syncs: Vec<SyncDir>) -> Result<(), Box<dyn Error>> {
    let data_folder = get_data_folder();
    let content = serde_json::to_string(&syncs)?;
    fs_store::update_file(data_folder, FILE::SYNCS, content)?;
    Ok(())
}