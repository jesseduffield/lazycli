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
  pub filter_text: String,
  pub focused_panel: FocusedPanel,
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
      filter_text: String::from(""),
      focused_panel: FocusedPanel::Table,
    }
  }

  pub fn filtered_rows(&self) -> Vec<&Row> {
    match self.filter_text.as_ref() {
      // TODO: ask if this is idiomatic rust: i.e. converting a Vec<Row> to Vec<&Row>
      "" => self.rows.iter().collect(),
      _ => self
        .rows
        .iter()
        .filter(|row| row.original_line.contains(&self.filter_text))
        .collect(),
    }
  }

  pub fn get_selected_row(&self) -> Option<&Row> {
    let selected_index = self.table.state.selected().unwrap();

    Some(*self.filtered_rows().get(selected_index)?)
  }

  pub fn adjust_cursor(&mut self) {
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

  pub fn update_rows(&mut self, rows: Vec<Row>) {
    self.rows = rows;
    self.adjust_cursor();
  }

  pub fn on_tick(&mut self) {
    // do nothing for now
  }

  pub fn push_filter_text_char(&mut self, c: char) {
    self.filter_text.push(c);
    self.adjust_cursor();
  }

  pub fn pop_filter_text_char(&mut self) {
    self.filter_text.pop();
    self.adjust_cursor();
  }

  pub fn reset_filter_text(&mut self) {
    self.filter_text = String::from("");
    self.adjust_cursor();
  }
}
