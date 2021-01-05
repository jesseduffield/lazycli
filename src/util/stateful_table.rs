use tui::widgets::TableState;

pub struct StatefulTable {
  pub state: TableState,
  pub rows: Vec<Vec<String>>,
}

impl StatefulTable {
  pub fn new(rows: Vec<Vec<String>>) -> StatefulTable {
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
