use std::fs;
use std::path::PathBuf;
use std::os::unix::fs::PermissionsExt;

pub fn copy_file(from_file: &PathBuf, to_file: &PathBuf) -> Result<()> {
    let operation = fs::copy(from_file, to_file).map(|_| ());
    if operation.is_ok() {
        let mut permissions = to_file.metadata()?.permissions();
        permissions.set_mode(777);
    }
}