use std::env;
#[allow(dead_code)]
use std::error::Error;

mod app;
mod args;
mod command;
mod config;
mod event_loop;
mod os_commands;
mod parse;
mod stateful_table;
mod template;
mod terminal_manager;
mod ui;

use app::App;
use args::Args;
use config::storage;
use storage::CONFIG_DIR_ENV_VAR;

fn main() -> Result<(), Box<dyn Error>> {
  let args = Args::new();
  let config_path = storage::config_path(env::var(CONFIG_DIR_ENV_VAR).ok())?;
  let config = storage::prepare_config(&config_path)?;

  let app = App::new(&config, config_path, args);

  event_loop::run(app)?;

  Ok(())
}
