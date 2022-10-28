#[cfg(not(debug_assertions))]
use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=client");
    build_client();
}

#[cfg(not(debug_assertions))]
fn build_client() {
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
fn build_client() {
    // skip in debug builds
}