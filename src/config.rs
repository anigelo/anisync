mod sync_dir;
mod search_filters;
mod mega_config;

use std::{env, error::Error, path::PathBuf};
use serde::{Serialize, Deserialize};
use crate::fs_store::{self,FILE};

pub use sync_dir::*;
pub use search_filters::*;
pub use mega_config::*;

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