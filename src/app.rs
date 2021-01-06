#[allow(dead_code)]
use crate::parse::Row;
use crate::util::stateful_table::StatefulTable;

pub struct App {
  pub rows: Vec<Row>,
  pub table: StatefulTable,
}

impl App {
  pub fn new(rows: Vec<Row>) -> App {
    App {
      table: StatefulTable::new(rows.len()),
      rows: rows,
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
}
