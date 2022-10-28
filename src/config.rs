use std::env;
use std::error::Error;
use std::path::PathBuf;
use crate::fs_store;
use serde::{Serialize, Deserialize};
use crate::fs_store::FILE;

#[derive(Deserialize, Serialize, Debug)]
pub struct Config {
    #[serde(rename = "MEGA_USER")]
    pub mega_user: Option<String>,
    #[serde(rename = "MEGA_PWD")]
    pub mega_pwd: Option<String>,
    #[serde(rename = "DOWNLOAD_FOLDER", default = "get_data_folder")]
    pub download_folder: PathBuf,
    #[serde(rename = "LOCAL_MEDIA_ROOT", default = "default_media")]
    pub local_media_root: PathBuf,
    #[serde(rename = "REMOTE_MEDIA_ROOT")]
    pub remote_media_root: Option<String>,
    #[serde(rename = "SCAN_INTERVAL", default = "default_interval")]
    pub scan_interval: u32
}

fn default_interval() -> u32 {
    86400
}

fn default_media() -> PathBuf {
    "/media".into()
}

fn get_data_folder() -> PathBuf {
    env::var("DATA_FOLDER").unwrap_or("/data".to_string()).into()
}

impl Config {
    pub fn read() -> Config {
        let data_folder = get_data_folder();
        let content = fs_store::read_file(data_folder, FILE::CONFIG);
        serde_json::from_str(&content.unwrap_or("{}".to_string())).unwrap()
    }
    pub fn persist(&self) -> Result<(), Box<dyn Error>> {
        let data_folder = get_data_folder();
        let content = serde_json::to_string(self)?;
        fs_store::update_file(data_folder, FILE::CONFIG, content)?;
        Ok(())
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct SyncDir {
    pub remote: String,
    pub local: PathBuf
}

impl SyncDir {
    pub fn remote_abs(&self, remote_root: &str) -> String {
        format!("{}{}", remote_root, &self.remote)
    }
    pub fn local_abs(&self, local_root: &PathBuf) -> PathBuf {
        local_root.join(&self.local)
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

#[derive(Deserialize, Serialize, Debug)]
pub struct SearchFilters {
    pub contains: Vec<String>,
    pub not_contains: Vec<String>
}

pub fn get_filters() -> SearchFilters {
    let content = fs_store::read_file(get_data_folder(), FILE::FILTERS);
    return serde_json::from_str(&content.unwrap_or("{}".to_string())).unwrap_or(SearchFilters {
        not_contains: vec![], contains: vec![]
    });
}

pub fn update_filters(filters: SearchFilters) -> Result<(), Box<dyn Error>> {
    let data_folder = get_data_folder();
    let content = serde_json::to_string(&filters)?;
    fs_store::update_file(data_folder, FILE::FILTERS, content)?;
    Ok(())
}