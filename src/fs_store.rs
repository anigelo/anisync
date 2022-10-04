use std::fs;
use std::io;
use std::path::PathBuf;

const CONFIG_FILE: &'static str = "config";
const SYNCS_FILE: &'static str = "syncs";
const FILTERS_FILE: &'static str = "filters";

pub enum FILE {
    CONFIG,
    SYNCS,
    FILTERS
}

pub fn read_file(data_root: PathBuf, file: FILE) -> Option<String> {
    let path = get_path(data_root, file);
    fs::read_to_string(&path)
        .map_err(|e| eprintln!("{:?}: {}", path, e))
        .ok()
}

pub fn update_file(data_root: PathBuf, file: FILE, content: String) -> io::Result<()> {
    let path = get_path(data_root, file);
    fs::write(path, content)
}

fn get_path(data_root: PathBuf, file: FILE) -> PathBuf {
    data_root.join(match file {
        FILE::CONFIG => CONFIG_FILE,
        FILE::SYNCS => SYNCS_FILE,
        FILE::FILTERS => FILTERS_FILE
    })
}