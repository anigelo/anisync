use std::{env, fs};
use std::path::PathBuf;
use std::os::unix::fs::PermissionsExt;
use file_owner::PathExt;

pub fn copy_file(from_file: &PathBuf, to_file: &PathBuf) -> std::io::Result<()> {
    fs::copy(from_file, to_file)?;
    try_own_file(to_file);

    let permissions = fs::Permissions::from_mode(0o777);
    fs::set_permissions(to_file, permissions)
}

fn try_own_file(file: &PathBuf) {
    let owner = env::var("UNIX_USER").ok()
        .and_then(|uid| uid.parse::<u32>().ok());
    if let Some(owner) = owner {
        if let Err(e) = file.set_owner(owner) {
            eprintln!("Error trying to own '{:?}': {:?}", file, e)
        }
    }

    let group = env::var("UNIX_GROUP").ok()
        .and_then(|gid| gid.parse::<u32>().ok());
    if let Some(group) = group {
        if let Err(e) = file.set_group(group) {
            eprintln!("Error trying to group own '{:?}': {:?}", file, e)
        }
    }
}