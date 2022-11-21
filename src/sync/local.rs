use std::error::Error;
use std::path::PathBuf;

pub fn next_local_episode(local_dir: &PathBuf) -> Result<u8, Box<dyn Error>> {
    let last_episode = std::fs::read_dir(local_dir)
        .map(|dir| dir
            .filter_map(|entry| entry.ok())
            .map(|entry| entry.path())
            .filter(|entry| entry.is_file())
            .filter_map(|file| file
                .file_stem().unwrap_or_default()
                .to_string_lossy()
                .parse::<u8>()
                .ok())
            .max()
            .unwrap_or(0)
        );

    Ok(last_episode? + 1)
}

pub fn move_to_media_dir(temp_path: PathBuf, new_path: PathBuf, ep_number: u8) -> Result<PathBuf, Box<dyn Error>> {
    let extension = new_path.extension()
        .map(|ext| ext.to_str())
        .flatten()
        .ok_or(format!("File '{}' has no extension", new_path.to_string_lossy()))?;

    let new_name = new_path.with_file_name(
        format!("{:0>2}.{}", ep_number, extension)
    );
    if super::sys::copy_file(&temp_path, &new_name).is_ok() {
        std::fs::remove_file(&temp_path)?;
    }
    Ok(new_name)
}