use std::{path::PathBuf};
use std::fmt::Debug;
use crate::config::{self,Config};

mod filtering;
mod mega;
mod sys;
mod local;

pub fn run(settings: Config) {
    if let Some(mega_settings) = settings.mega().log_error() {
        if mega::login(&mega_settings.user, &mega_settings.password).log_output() {
            sync_folders(&settings.download_folder, &settings.local_media_root, mega_settings.remote);
        }
    }
}

fn sync_folders(download_dir: &PathBuf, local_media_root: &PathBuf, remote_root: String) {
    for sync_dir in config::get_sync_folders() {
        let local = sync_dir.local_abs(local_media_root).with_log("Local:");
        let remote = sync_dir.remote_abs(&remote_root).with_log("Remote:");

        let remote_episodes = mega::ls(&remote).log_error().unwrap_or_default();
        let next_local_episode = local::next_local_episode(&local).log_error().unwrap_or_default();

        if let Some(media) = filtering::find_closest_episode(remote_episodes, next_local_episode) {
            mega::get(&format!("{}/{}", remote, media), &download_dir).log_output();

            local::move_to_media_dir(
                download_dir.join(&media),
                PathBuf::from(local).join(&media),
                next_local_episode).to_log("Moved to");
        }
    }
}

trait ResultLogExt<T: Debug> {
    fn log_error(self) -> Option<T>;
    fn to_log(self, prefix: &str);
}

trait OutputLogExt {
    fn log_output(self) -> bool;
}

trait ToLogExt<T: Debug> {
    fn with_log(self, prefix: &str) -> T;
}

impl<T: Debug> ToLogExt<T> for T {
    fn with_log(self, prefix: &str) -> T {
        println!("{} {:?}", prefix, self);
        self
    }
}

impl<E: Debug> OutputLogExt for Result<String, E> {
    fn log_output(self) -> bool {
        match self {
            Ok(log) => {
                println!("{}", log);
                true
            },
            Err(err) => {
                eprintln!("{:?}", err);
                false
            }
        }
    }
}

impl<T: Debug, E: Debug> ResultLogExt<T> for Result<T, E> {
    fn log_error(self) -> Option<T> {
        match self {
            Ok(value) => Some(value),
            Err(err) => {
                eprintln!("{:?}", err);
                None
            }
        }
    }

    fn to_log(self, prefix: &str) {
        match self {
            Ok(value) => println!("{} {:?}", prefix, value),
            Err(e) => eprintln!("{:?}", e)
        }
    }
}