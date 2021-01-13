use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;
use xdg::BaseDirectories;

use super::Config;

pub fn config_path() -> Result<PathBuf, Box<dyn Error>> {
  let xdg_dirs = BaseDirectories::with_prefix("lazycli")?;

  Ok(xdg_dirs.place_config_file("config.yml")?)
}

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
