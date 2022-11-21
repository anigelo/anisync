use std::{path::PathBuf};
use crate::config::{self,Config};

mod filtering;
mod mega;
mod sys;
mod local;

pub fn run_sync(settings: Config) {
    if let (Some(mega_pwd), Some(mega_user), Some(remote_root)) = (settings.mega_pwd, settings.mega_user, settings.remote_media_root) {
        match mega::login(&mega_user, &mega_pwd) {
            Ok(_) => sync_folders(&settings.download_folder, &settings.local_media_root, remote_root),
            Err(e) => eprintln!("Login error: {:?}", e),
        }
    } else {
        eprintln!("MEGA credentials and remote directory root are required");
    }
}

fn sync_folders(download_dir: &PathBuf, local_media_root: &PathBuf, remote_root: String) {
    for sync_dir in config::get_sync_folders() {
        sync_folder(
            download_dir,
            sync_dir.local_abs(local_media_root),
            sync_dir.remote_abs(&remote_root)
        )
    }
}

fn sync_folder(download_folder: &PathBuf, local: PathBuf, remote: String) {
    println!("Local: {:?}\nRemote: {:?}", local, remote);
    let remote_episodes = list_remote_folder(&remote);
    let next_local_episode = local::next_local_episode(&local).unwrap();
    println!("Next episode: {}", next_local_episode);

    if let Some(media) = filtering::find_closest_episode(remote_episodes, next_local_episode) {
        println!("Match: {}", media);
        println!("Download folder: {:?}", download_folder);

        mega::get(&format!("{}/{}", remote, media), &download_folder)
            .unwrap();

        local::move_to_media_dir(
            download_folder.join(&media),
            PathBuf::from(local).join(&media),
            next_local_episode).unwrap();
    } else {
        println!("Up to date for {:?}", local);
    }
    println!();
}

fn list_remote_folder(remote_folder: &str) -> Vec<String> {
    match mega::ls(remote_folder) {
        Ok(list) => list,
        Err(e) => {
            eprintln!("{:?}", e);
            vec![]
        }
    }
}