use std::path::PathBuf;

use crate::{
  args::Args,
  command,
  config::{Config, Profile},
  parse::Row,
  profile_matching::command_matches,
  stateful_table::StatefulTable,
  template,
};

#[derive(PartialEq)]
pub enum FocusedPanel {
  Table,
  Search,
  // TODO: should I store the error here in the enum, given
  // it isn't needed anywhere else, and only applies to that panel?
  ErrorPopup(String),
  ConfirmationPopup(String),
}

pub struct App<'a> {
  pub rows: Vec<Row>,
  pub table: StatefulTable,
  pub config: &'a Config,
  pub profile: Option<&'a Profile>,
  pub args: Args,
  pub status_text: Option<String>,
  pub filter_text: String,
  pub focused_panel: FocusedPanel,
  pub selected_item_content: String,
  pub config_path: PathBuf,
}

impl<'a> App<'a> {
  // TODO: do we really need a reference to the config? We should probably move it in here. But then we need to still work out how to have a profile field. We could either make that a function or make it an immutable reference
  pub fn new(config: &'a Config, config_path: PathBuf, args: Args) -> App<'a> {
    let profile = config.profiles.iter().find(|p| {
      p.registered_commands
        .iter()
        .any(|c| command_matches(args.command.as_ref(), c))
    });

    App {
      table: StatefulTable::new(0),
      rows: vec![],
      config,
      profile,
      args,
      status_text: None,
      filter_text: String::from(""),
      focused_panel: FocusedPanel::Table,
      selected_item_content: String::from(""),
      config_path,
    }
  }

  pub fn on_select(&mut self) -> Option<()> {
    let selected_row = self.get_selected_row()?;
    let command_template = self.profile?.display_command.as_ref()?;
    let command = template::resolve_command(command_template, selected_row);

    let output = command::run_command(&command).unwrap();
    self.selected_item_content = output;

    Some(())
  }

  pub fn filtered_rows(&self) -> Vec<&Row> {
    let lc_filter_text = self.filter_text.to_ascii_lowercase();

    match self.filter_text.as_ref() {
      // TODO: ask if this is idiomatic rust: i.e. converting a Vec<Row> to Vec<&Row>
      "" => self.rows.iter().collect(),
      _ => self
        .rows
        .iter()
        .filter(|row| {
          row
            .original_line
            .to_ascii_lowercase()
            .contains(&lc_filter_text)
        })
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
