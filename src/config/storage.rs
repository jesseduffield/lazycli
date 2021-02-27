use std::{error::Error, fs::File, io::prelude::*, path::PathBuf};

use super::Config;

use std::fs;
use std::io;
use std::path::Path;

extern crate directories;
use directories::ProjectDirs;

// adapted from xdg crate
fn write_file<P>(home: &PathBuf, path: P) -> io::Result<PathBuf>
where
  P: AsRef<Path>,
{
  match path.as_ref().parent() {
    Some(parent) => (fs::create_dir_all(home.join(parent)))?,
    None => (fs::create_dir_all(home))?,
  }
  Ok(PathBuf::from(home.join(path.as_ref())))
}

pub fn config_path() -> Result<PathBuf, Box<dyn Error>> {
  let config_dir = ProjectDirs::from("", "", "lazycli")
    .unwrap()
    .config_dir()
    .to_owned();

  Ok(write_file(&PathBuf::from(config_dir), "config.yml")?)
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
