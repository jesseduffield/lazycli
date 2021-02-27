use std::{error::Error, fs::File, io::prelude::*, path::PathBuf};

use super::Config;

#[cfg_attr(target_os = "linux", path = "windows.rs")]
#[cfg_attr(target_os = "macos", path = "windows.rs")]
#[cfg_attr(windows, path = "windows.rs")]
pub mod os;

pub fn prepare_config(config_path: &PathBuf) -> Result<Config, Box<dyn Error>> {
  if config_path.exists() {
    let mut file = File::open(config_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(Config::from_yaml(contents)?)
  } else {
    let default_config_yml = Config::new().to_yaml().unwrap();
    let config = Config::new();
    let mut file = File::create(config_path)?;
    file.write_all(default_config_yml.as_bytes())?;
    Ok(config)
  }
}
