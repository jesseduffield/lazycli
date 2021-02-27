use std::fs;
use std::io;
use std::path::Path;

extern crate directories;
use directories::ProjectDirs;

use std::{error::Error, path::PathBuf};

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
