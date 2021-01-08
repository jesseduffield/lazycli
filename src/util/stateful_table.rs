use tui::widgets::TableState;

pub struct StatefulTable {
  pub state: TableState,
  pub row_count: usize,
}

impl StatefulTable {
  pub fn new(row_count: usize) -> StatefulTable {
    StatefulTable {
      state: TableState::default(),
      row_count,
    }
  }

  pub fn next(&mut self) {
    let i = match self.state.selected() {
      Some(i) => {
        if self.row_count == 0 || i >= self.row_count - 1 {
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
