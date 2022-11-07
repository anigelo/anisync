use std::{path::PathBuf};
use crate::config::{self,Config};
use sys::copy_file;

mod filtering;
mod mega;
mod sys;

pub fn run_sync(settings: Config) {
    if let (Some(mega_pwd), Some(mega_user), Some(remote_root)) = (settings.mega_pwd, settings.mega_user, settings.remote_media_root) {
        match mega::login(&mega_user, &mega_pwd) {
            Ok(output) => {
                println!("{}", output);
                for sync_dir in config::get_sync_folders() {
                    sync_folder(
                            &settings.download_folder,
                    sync_dir.local_abs(&settings.local_media_root),
                    sync_dir.remote_abs(&remote_root)
                    )
                }
            },
            Err(e) => eprintln!("Login error: {:?}", e),
        }
    } else {
        eprintln!("MEGA credentials and remote directory root are required");
    }
}

fn sync_folder(download_folder: &PathBuf, local: PathBuf, remote: String) {
    println!("Local: {:?}\nRemote: {:?}", local, remote);
    let remote_episodes = list_remote_folder(&remote);
    let next_local_episode = next_local_episode(&local);
    println!("Next episode: {}", next_local_episode);

    if let Some(media) = filtering::find_closest_episode(remote_episodes, next_local_episode) {
        println!("Match: {}", media);
        println!("Download folder: {:?}", download_folder);

        mega::get(&format!("{}/{}", remote, media), &download_folder)
            .unwrap();

        let temp_path = download_folder.join(&media);

        let new_path = PathBuf::from(local).join(&media);
        let extension = new_path.extension().unwrap().to_str().unwrap();
        let new_name = new_path.with_file_name(format!("{:0>2}.{}", next_local_episode, extension));
        println!("Temp path: {:?}", temp_path);
        println!("Destination: {:?}", new_name);

        if copy_file(&temp_path, &new_name).is_ok() {
            std::fs::remove_file(&temp_path).unwrap();
        }

    } else {
        println!("Up to date for {:?}", local);
    }
    println!();
}

fn next_local_episode(local_folder: &PathBuf) -> u8 {
    let last_episode = std::fs::read_dir(local_folder).expect("Invalid path")
    .filter_map(|entry| entry.ok())
    .map(|entry| entry.path())
    .filter(|entry| entry.is_file())
    .filter_map(|file| file.file_stem().unwrap().to_str().unwrap().parse::<u8>().ok())
    .max().unwrap_or(0);

    last_episode + 1
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