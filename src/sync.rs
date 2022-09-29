use std::path::PathBuf;
use std::process::{Command, ExitStatus};
use crate::config;

pub fn run_sync() {
    let exit_status = try_login();
    println!();

    if exit_status.success() || exit_status.code().unwrap() == 54 {
        for (remote, local) in config::get_sync_folders() {
            println!("Local: {:?}\nRemote: {:?}", local, remote);
            let remote_episodes = list_remote_folder(&remote);
            let next_local_episode = next_local_episode(&local);
            println!("Next episode: {}", next_local_episode);

            let match_episode = remote_episodes.into_iter()
                .find(|episode| episode.contains(&format!(" {:0>2}.", next_local_episode)) && apply_filters(episode));

            if let Some(media) = match_episode {
                println!("Match: {}", media);
                let download_folder = config::get_download_folder();
                println!("Download folder: {:?}", download_folder);

                Command::new(adapt_to_os("mega-get"))
                    .arg(format!("{}/{}", remote, media))
                    .arg(&download_folder)
                    .spawn().unwrap()
                    .wait().unwrap();

                let temp_path = download_folder.join(&media);

                let new_path = PathBuf::from(local).join(&media);
                let extension = new_path.extension().unwrap().to_str().unwrap();
                let new_name = new_path.with_file_name(format!("{:0>2}.{}", next_local_episode, extension));
                println!("Temp path: {:?}", temp_path);
                println!("Destination: {:?}", new_name);

                std::fs::copy(&temp_path, new_name).unwrap();
                std::fs::remove_file(&temp_path).unwrap();
            } else {
                println!("Up to date for {:?}", local);
            }
            println!();
        }
    }
}

fn try_login() -> ExitStatus {
    let (user, password) = config::get_mega_credentials();

    Command::new(adapt_to_os("mega-login"))
        .arg(user)
        .arg(password)
        .status().unwrap()
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
    let output = Command::new(adapt_to_os("mega-ls"))
        .arg(remote_folder)
        .output();

    match output {
        Ok(output) => {
            let ls = String::from_utf8_lossy(&output.stdout);
            ls.lines()
                .map(|line| line.to_string())
                .collect()
        },
        Err(e) => {
            eprintln!("Error on 'mega-ls' for path '{}', Error: {:#?}", remote_folder, e);
            vec![]
        }
    }
}

fn apply_filters(episode: &str) -> bool {
    let filters = config::get_filters();
    let banned_terms = config::get_banned_terms();

    for filter in filters {
        if !episode.contains(&filter) {
            return false;
        }
    }

    for banned_term in banned_terms {
        if episode.contains(&banned_term) {
            return false;
        }
    }

    true
}

fn adapt_to_os(command: &str) -> String {
    if config::is_windows() {
        format!("{}.bat", command)
    } else {
        command.to_string()
    }
}