use std::{error::Error, fs::File, io::prelude::*, path::PathBuf};

use super::Config;

use std::{fs, io, path::Path};

extern crate directories;
use directories::ProjectDirs;

pub const CONFIG_DIR_ENV_VAR: &str = "LAZYCLI_CONFIG_DIR";

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

pub fn config_path(dir_env_var: Option<String>) -> Result<PathBuf, Box<dyn Error>> {
  let config_dir = config_dir(dir_env_var);

  Ok(write_file(&PathBuf::from(config_dir), "config.yml")?)
}

fn config_dir(dir_env_var: Option<String>) -> PathBuf {
  match dir_env_var {
    Some(value) => PathBuf::from(value),
    None => ProjectDirs::from("", "", "lazycli")
      .unwrap()
      .config_dir()
      .to_owned(),
  }
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

#[cfg(test)]
mod tests {
  use super::*;
  use pretty_assertions::assert_eq;

  #[test]
  fn test_use_config_dir_when_provided() {
    assert_eq!(
      config_dir(Some(String::from("/mydir"))),
      PathBuf::from("/mydir")
    )
  }

  #[test]
  fn test_fallback_to_default_config_dir() {
    let result = config_dir(None);
    println!("{:?}", result);
    assert!(
      // not asserting on the whole path given that it's platform-dependent
      result.ends_with("lazycli"),
      "should end in 'lazycli'!"
    );
  }
}
