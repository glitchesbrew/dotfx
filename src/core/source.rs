use serde::{Deserialize, Serialize};
use std::path::Path;
use std::fs;


#[derive(Deserialize, Debug, Serialize)]
pub struct DotfilesConfig {
    path_to_dotfiles: String,
    username: String,
    git_conf_loc: String,
}

pub fn get_config() -> Result<DotfilesConfig, &'static str> {
    match self::DotfilesConfig::from_file() {
        Ok(v) => Ok(v),
        Err(e) => Err(e),
    }
}

impl DotfilesConfig {

    const GIT_FOLDER: &'static str = "/.git";
    const CONFIG_LOCATION: &'static str = "~/.config/dotfx/config.toml";
    const DEFAULT_CONFIG_LOCATION: &'static str = "./config.toml";

    pub fn new(path_to_dotfiles: String, username: String, git_conf_loc: String) -> Result<DotfilesConfig, &'static str> {
        let new_conf = self::DotfilesConfig{
            path_to_dotfiles,
            username,
            git_conf_loc,
        };

        let is_gd: bool = new_conf.is_git_dir();
        let result = match is_gd {
            true => Ok(new_conf),
            false => Err("dotfile dir must be a git directory"),
        };

        return result;
    }

    fn from_file() -> Result<DotfilesConfig, &'static str> {
        let config_text: String;
        match fs::read_to_string(Self::CONFIG_LOCATION) {
            Ok(v) => config_text = v,
            Err(_) => return Err("error reading file")
        }

        match toml::from_str(&config_text) {
            Ok(v) => Ok(v),
            Err(_) => Err("toml file is misconfigured"),
        }
    }

    fn is_git_dir(&self) -> bool {
        let mut dot_path: String = String::to_owned(&self.path_to_dotfiles);
        dot_path.push_str(Self::GIT_FOLDER);
        let is_git_dir: bool = Path::new(dot_path.as_str()).exists();

        return is_git_dir;
    }

    fn create_config_file() -> Result<&'static str, &'static str> {
        let res = fs::copy(Self::DEFAULT_CONFIG_LOCATION, Self::CONFIG_LOCATION);
        match res {
            Ok(_) => Ok(Self::CONFIG_LOCATION),
            Err(_) => Err("an error occurred while creating config file"),
        }
    }
}
