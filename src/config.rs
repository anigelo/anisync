use std::env;
use std::path::PathBuf;

pub fn get_mega_credentials() -> (String, String) {
    let user = env::var("MEGA_USER").expect("MEGA_USER not present");
    let password = env::var("MEGA_PWD").expect("MEGA_PWD not present");

    (user, password)
}

pub fn is_windows() -> bool {
    env::var("IS_WINDOWS").is_ok()
}

pub fn get_download_folder() -> PathBuf {
    env::var("DOWNLOAD_FOLDER").unwrap_or("/media".to_string()).into()
}

pub fn get_sync_folders() -> Vec<(String, PathBuf)> {
    let remote_folders = get_env_collection("MEGA_WATCH_FOLDERS");
    let local_folders = get_env_collection("LOCAL_MEDIA_FOLDERS");
    let local_base_folder: PathBuf = env::var("LOCAL_MEDIA_FOLDER").unwrap_or("/media".to_string()).into();

    if remote_folders.len() == local_folders.len() {
        remote_folders.into_iter().zip(
            local_folders.into_iter().map(|path| local_base_folder.join(path))
        ).collect()
    } else {
        vec![]
    }
}

pub fn get_filters() -> Vec<String> {
    get_env_collection("FILTER_WORDS")
}

pub fn get_banned_terms() -> Vec<String> {
    get_env_collection("BANNED_WORDS")
}

fn get_env_collection(env_var: &str) -> Vec<String> {
    env::var(env_var)
        .unwrap_or_default()
        .split(';')
        .map(|entry| entry.to_string())
        .collect::<Vec<String>>()
}