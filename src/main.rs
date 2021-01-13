#[allow(dead_code)]
use std::error::Error;

mod app;
mod args;
mod command;
mod config;
mod event_loop;
mod parse;
mod stateful_table;
mod template;
mod terminal_manager;
mod ui;

use app::App;
use args::Args;
use config::prepare;

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::new();
    let config = prepare::prepare_config()?;

    let app = App::new(&config, args);

    event_loop::run(app)?;

    Ok(())
}
