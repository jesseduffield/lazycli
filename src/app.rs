use crate::args::Args;
use crate::config::Config;
#[allow(dead_code)]
use crate::config::Profile;
use crate::parse::Row;
use crate::util::stateful_table::StatefulTable;

pub struct App<'a> {
  pub rows: Vec<Row>,
  pub table: StatefulTable,
  pub config: &'a Config,
  pub profile: Option<&'a Profile>,
  pub args: Args,
  pub should_quit: bool,
  pub is_loading: bool,
}

impl<'a> App<'a> {
  pub fn new(config: &'a Config, args: Args) -> App<'a> {
    let profile = config
      .profiles
      .iter()
      .find(|p| p.registered_commands.iter().any(|c| *c == args.command));

    App {
      table: StatefulTable::new(0),
      rows: vec![],
      config,
      profile,
      args,
      should_quit: false,
      is_loading: false,
    }
  }

  pub fn get_selected_row(&self) -> &Row {
    let selected_index = self.table.state.selected().unwrap();

    &self.rows[selected_index]
  }

  pub fn update_rows(&mut self, rows: Vec<Row>) {
    let length = rows.len();
    self.table.row_count = length;
    self.rows = rows;
    // if our cursor is too far we need to correct it
    if length == 0 {
      self.table.state.select(Some(0));
    } else if self.table.state.selected().unwrap() > length - 1 {
      self.table.state.select(Some(length - 1));
    }
  }

  pub fn on_tick(&mut self) {
    // do nothing for now
  }
}
