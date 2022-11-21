use std::fmt::{Display, Formatter};
use crate::config::Config;

#[derive(Debug)]
pub struct MegaConfig {
    pub user: String,
    pub password: String,
    pub remote: String
}

impl Config {
    pub fn mega(&self) -> Result<MegaConfig, MegaConfigError> {
        Ok(MegaConfig {
            user: self.mega_user.clone().ok_or(MegaConfigError::NoUser)?,
            password: self.mega_pwd.clone().ok_or(MegaConfigError::NoPassword)?,
            remote: self.remote_media_root.clone().ok_or(MegaConfigError::NoRemote)?
        })
    }
}

#[derive(Debug)]
pub enum MegaConfigError {
    NoUser, NoPassword, NoRemote
}

impl Display for MegaConfigError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            MegaConfigError::NoUser => write!(f, "No MEGA user present in the configuration"),
            MegaConfigError::NoPassword => write!(f, "No MEGA password present in the configuration"),
            MegaConfigError::NoRemote => write!(f, "No MEGA shared directory root present in the configuration")
        }
    }
}