use std::{error::Error, path::PathBuf};

use xdg::BaseDirectories;

pub fn config_path() -> Result<PathBuf, Box<dyn Error>> {
  let xdg_dirs = BaseDirectories::with_prefix("lazycli")?;

  Ok(xdg_dirs.place_config_file("config.yml")?)
}
