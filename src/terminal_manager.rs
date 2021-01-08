use crossterm::{
  event::{DisableMouseCapture, EnableMouseCapture},
  execute,
  terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::error::Error;
use std::io::stdout;
use tui::{backend::CrosstermBackend, Terminal};

pub struct TerminalManager {
  pub terminal: tui::Terminal<tui::backend::CrosstermBackend<std::io::Stdout>>,
}

impl TerminalManager {
  pub fn new() -> Result<TerminalManager, Box<dyn Error>> {
    enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    Ok(TerminalManager {
      terminal: Terminal::new(backend)?,
    })
  }

  pub fn teardown(&mut self) -> Result<(), Box<dyn Error>> {
    disable_raw_mode()?;
    execute!(
      self.terminal.backend_mut(),
      LeaveAlternateScreen,
      DisableMouseCapture
    )?;
    // TODO: understand why this works
    Ok(self.terminal.show_cursor()?)
  }
}
