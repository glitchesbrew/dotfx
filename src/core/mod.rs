pub mod resource;
pub mod nvim;
pub mod source;

pub fn load_config() -> Result<source::DotfilesConfig, &'static str>{
    match source::get_config() {
        Ok(dotfile_config) => Ok(dotfile_config),
        Err(msg) => Err(msg),
    }
}

