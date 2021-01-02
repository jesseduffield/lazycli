#[cfg(feature = "termion")]
use crate::util::event;
pub mod event;
use tui::widgets::ListState;

#[derive(Clone)]
pub struct SinSignal {
  x: f64,
  interval: f64,
  period: f64,
  scale: f64,
}

impl SinSignal {
  pub fn new(interval: f64, period: f64, scale: f64) -> SinSignal {
    SinSignal {
      x: 0.0,
      interval,
      period,
      scale,
    }
  }
}

impl Iterator for SinSignal {
  type Item = (f64, f64);
  fn next(&mut self) -> Option<Self::Item> {
    let point = (self.x, (self.x * 1.0 / self.period).sin() * self.scale);
    self.x += self.interval;
    Some(point)
  }
}

pub struct TabsState<'a> {
  pub titles: Vec<&'a str>,
  pub index: usize,
}

impl<'a> TabsState<'a> {
  pub fn new(titles: Vec<&'a str>) -> TabsState {
    TabsState { titles, index: 0 }
  }
  pub fn next(&mut self) {
    self.index = (self.index + 1) % self.titles.len();
  }

  pub fn previous(&mut self) {
    if self.index > 0 {
      self.index -= 1;
    } else {
      self.index = self.titles.len() - 1;
    }
  }
}

pub struct StatefulList<T> {
  pub state: ListState,
  pub items: Vec<T>,
}

impl<T> StatefulList<T> {
  pub fn new() -> StatefulList<T> {
    StatefulList {
      state: ListState::default(),
      items: Vec::new(),
    }
  }

  pub fn with_items(items: Vec<T>) -> StatefulList<T> {
    StatefulList {
      state: ListState::default(),
      items,
    }
  }

  pub fn next(&mut self) {
    let i = match self.state.selected() {
      Some(i) => {
        if i >= self.items.len() - 1 {
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

  pub fn unselect(&mut self) {
    self.state.select(None);
  }
}
