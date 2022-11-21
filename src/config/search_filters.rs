use std::error::Error;
use serde::{Serialize, Deserialize};
use crate::fs_store;
use crate::fs_store::FILE;
use super::get_data_folder;

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