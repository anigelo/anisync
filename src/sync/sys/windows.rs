use std::fs;
use std::path::PathBuf;
use std::io::Result;

pub fn copy_file(from_file: &PathBuf, to_file: &PathBuf) -> Result<()> {
    fs::copy(from_file, to_file).map(|_| ())
}