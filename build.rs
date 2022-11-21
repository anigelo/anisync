#[cfg(not(debug_assertions))]
use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=client");
    build();
}

#[cfg(not(debug_assertions))]
fn build() {
    let status = Command::new("npm")
        .arg("install")
        .current_dir("www")
        .status()
        .expect("failed to install client dependencies");
    assert!(status.success());

    let status = Command::new("npm")
        .arg("run")
        .arg("build")
        .current_dir("www")
        .status()
        .expect("failed to build client");
    assert!(status.success());
}

#[cfg(debug_assertions)]
fn build() {
    let path = std::path::PathBuf::from("test-media");
    if std::fs::create_dir(&path).is_ok() {
        create_test_env(path);
    }
}

#[cfg(debug_assertions)]
fn create_test_env(path: std::path::PathBuf) {
    let media = path.join("test");
    if std::fs::create_dir(&media).is_ok() {
        std::fs::write(media.join("01.mkv"), "").ok();
    }
}