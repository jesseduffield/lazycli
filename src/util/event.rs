use std::io;
use std::sync::mpsc;
use std::sync::{
  atomic::{AtomicBool, Ordering},
  Arc,
};
use std::thread;
use termion::event::Key;
use termion::input::TermRead;

pub enum Event<I> {
  Input(I),
}

/// A small event handler that wrap termion input and tick events. Each event
/// type is handled in its own thread and returned to a common `Receiver`
pub struct Events {
  rx: mpsc::Receiver<Event<Key>>,
  input_handle: thread::JoinHandle<()>,
  ignore_exit_key: Arc<AtomicBool>,
}

#[derive(Debug, Clone, Copy)]
pub struct Config {
  pub exit_key: Key,
}

impl Default for Config {
  fn default() -> Config {
    Config {
      exit_key: Key::Char('q'),
    }
  }
}

impl Events {
  pub fn new() -> Events {
    Events::with_config(Config::default())
  }

  pub fn with_config(config: Config) -> Events {
    let (tx, rx) = mpsc::channel();
    let ignore_exit_key = Arc::new(AtomicBool::new(false));
    let input_handle = {
      let tx = tx.clone();
      let ignore_exit_key = ignore_exit_key.clone();
      thread::spawn(move || {
        let stdin = io::stdin();
        for evt in stdin.keys() {
          if let Ok(key) = evt {
            if let Err(err) = tx.send(Event::Input(key)) {
              eprintln!("{}", err);
              return;
            }
            if !ignore_exit_key.load(Ordering::Relaxed) && key == config.exit_key {
              return;
            }
          }
        }
      })
    };
    Events {
      rx,
      ignore_exit_key,
      input_handle,
    }
  }

  pub fn next(&self) -> Result<Event<Key>, mpsc::RecvError> {
    self.rx.recv()
  }

  pub fn disable_exit_key(&mut self) {
    self.ignore_exit_key.store(true, Ordering::Relaxed);
  }

  pub fn enable_exit_key(&mut self) {
    self.ignore_exit_key.store(false, Ordering::Relaxed);
  }
}
