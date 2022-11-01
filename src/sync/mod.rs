use std::error::Error;
use std::path::PathBuf;
use std::process::{Command, ExitStatus};
use crate::config;
use crate::config::Config;

mod filtering;

pub fn run_sync(settings: Config) {
    if let (Some(mega_pwd), Some(mega_user), Some(remote_root)) = (settings.mega_pwd, settings.mega_user, settings.remote_media_root) {
        let exit_status = try_login(mega_user, mega_pwd);
        println!();

        match exit_status {
            Ok(status) if is_logged_in(status) => {
                for sync_dir in config::get_sync_folders() {
                    sync_folder(
                            &settings.download_folder,
                    sync_dir.local_abs(&settings.local_media_root),
                    sync_dir.remote_abs(&remote_root)
                    )
                }
            },
            Err(e) => eprintln!("Login error: {:?}", e),
            _ => println!("Unknown login error")
        }
    } else {
        eprintln!("MEGA credentials and remote directory root are required");
    }
}

fn is_logged_in(exit_status: ExitStatus) -> bool {
    exit_status.success() || exit_status.code().unwrap_or(-1) == 54
}

fn sync_folder(download_folder: &PathBuf, local: PathBuf, remote: String) {
    println!("Local: {:?}\nRemote: {:?}", local, remote);
    let remote_episodes = list_remote_folder(&remote);
    let next_local_episode = next_local_episode(&local);
    println!("Next episode: {}", next_local_episode);

    if let Some(media) = filtering::find_closest_episode(remote_episodes, next_local_episode) {
        println!("Match: {}", media);
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

fn try_login(user: String, password: String) -> Result<ExitStatus, Box<dyn Error>> {
    let status = Command::new(adapt_to_os("mega-login"))
    .arg(user)
    .arg(password)
    .status()?;

    Ok(status)
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
        Ok(output) if output.status.success() => {
            let ls = String::from_utf8_lossy(&output.stdout);
            ls.lines()
            .map(|line| line.to_string())
            .collect()
        },
        Ok(output_error) => {
            let error = String::from_utf8_lossy(&output_error.stdout);
            eprintln!("Error on 'mega-ls' for path '{}'", remote_folder);
            println!("{}", error);
            vec![]
        },
        Err(e) => {
            eprintln!("Error on 'mega-ls' for path '{}', Error: {:#?}", remote_folder, e);
            vec![]
        }
    }
}

fn adapt_to_os(command: &str) -> String {
    if cfg!(windows) {
        format!("{}.bat", command)
    } else {
        command.to_string()
    }
}