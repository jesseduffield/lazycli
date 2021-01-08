use crate::args::Args;
use crate::config::Config;
#[allow(dead_code)]
use crate::config::Profile;
use crate::parse::Row;
use crate::util::stateful_table::StatefulTable;

#[derive(PartialEq)]
pub enum FocusedPanel {
  Table,
  Search,
}

pub struct App<'a> {
  pub rows: Vec<Row>,
  pub table: StatefulTable,
  pub config: &'a Config,
  pub profile: Option<&'a Profile>,
  pub args: Args,
  pub should_quit: bool,
  pub status_text: Option<String>,
  pub filter_text: Option<String>,
  pub focused_panel: FocusedPanel,
  pub search_text: String,
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
      status_text: None,
      filter_text: None,
      focused_panel: FocusedPanel::Table,
      search_text: String::from(""),
    }
  }

  pub fn filtered_rows(&self) -> Vec<&Row> {
    match &self.filter_text {
      // TODO: ask if this is idiomatic rust: i.e. converting a Vec<Row> to Vec<&Row>
      None => self.rows.iter().collect(),
      Some(filter_text) => self
        .rows
        .iter()
        .filter(|row| row.original_line.contains(filter_text))
        .collect(),
    }
  }

  pub fn get_selected_row(&self) -> &Row {
    let selected_index = self.table.state.selected().unwrap();

    &self.filtered_rows()[selected_index]
  }

  pub fn update_rows(&mut self, rows: Vec<Row>) {
    self.rows = rows;
    let filtered_rows = self.filtered_rows();
    let length = filtered_rows.len();
    self.table.row_count = length;
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
