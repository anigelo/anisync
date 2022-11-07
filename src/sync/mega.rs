use std::error::Error;
use std::path::PathBuf;
use std::process::{Command, ExitStatus};

mod sys;

pub fn login(user: &str, password: &str) -> Result<String, Box<dyn Error>> {
    let command: String = Mega::LOGIN.into();
    let output = Command::new(command)
        .args([user, password])
        .output()?;

    let output = if is_logged_in(output.status) {
        Ok(String::from_utf8(output.stdout)?)
    } else {
        Err(String::from_utf8(output.stderr)?)
    }?;

    Ok(output)
}

pub fn get(remote_media: &str, download_dir: &PathBuf) -> Result<(), Box<dyn Error>> {
    let command: String = Mega::GET.into();
    let status = Command::new(command)
        .arg(remote_media)
        .arg(download_dir)
        .spawn()?
        .wait()?;

    if !status.success() {
        Err(format!("ExitStatus: {:#?}", status))?;
    }

    Ok(())
}

pub fn ls(remote_dir: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let command: String = Mega::LS.into();
    let output = Command::new(command)
        .arg(remote_dir)
        .output();

    let output = match output {
        Ok(output) if output.status.success() => {
            let ls = String::from_utf8(output.stdout)?
                .lines()
                .map(|line| line.to_string())
                .collect();
            Ok(ls)
        },
        Ok(output_error) => {
            let error = String::from_utf8(output_error.stderr)?;
            Err(format!("Error on 'mega-ls' for path '{}', Error: {:#?}", remote_dir, error))
        },
        Err(e) => {
            Err(format!("Error on 'mega-ls' for path '{}', Error: {:#?}", remote_dir, e))
        }
    }?;

    Ok(output)
}

enum Mega {
    LS, GET, LOGIN
}

fn is_logged_in(exit_status: ExitStatus) -> bool {
    exit_status.success() || exit_status.code().unwrap_or(-1) == 54
}