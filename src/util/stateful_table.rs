use tui::widgets::TableState;

pub struct StatefulTable<'a> {
  pub state: TableState,
  pub rows: Vec<Vec<&'a str>>,
}

impl<'a> StatefulTable<'a> {
  pub fn new(rows: Vec<Vec<&'a str>>) -> StatefulTable<'a> {
    StatefulTable {
      state: TableState::default(),
      rows,
    }
  }

  pub fn next(&mut self) {
    let i = match self.state.selected() {
      Some(i) => {
        if i >= self.rows.len() - 1 {
          i
        } else {
          i + 1
        }
      }
      None => 0,
    };
    self.state.select(Some(i));
  }

  pub fn previous(&mut self) {
    let i = match self.state.selected() {
      Some(i) => {
        if i == 0 {
          i
        } else {
          i - 1
        }
      }
      None => 0,
    };
    self.state.select(Some(i));
  }
}
