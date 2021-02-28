use crossterm::event::{self, Event as CEvent, KeyCode, KeyEvent, KeyModifiers};
use std::{
  error::Error,
  sync::mpsc::{self, Receiver, Sender},
  thread,
  time::{Duration, Instant},
};

use ticker::Ticker;

use crate::{
  app::{App, FocusedPanel},
  command, os_commands,
  parse::{self, Row},
  template,
  terminal_manager::TerminalManager,
  ui,
};

enum Event<I> {
  Input(I),
  Tick,
  RefetchData(bool), // the bool here is true if it's a background refetch
  RowsLoaded(Vec<Row>),
  Error(String),
}

pub fn run(mut app: App) -> Result<(), Box<dyn Error>> {
  // select the first row (no rows will be loaded at this point but that's okay)
  app.table.next();

  let lines_to_skip = if app.args.lines_to_skip != 0 {
    app.args.lines_to_skip
  } else {
    match app.profile {
      Some(profile) => profile.lines_to_skip,
      None => 0,
    }
  };

  // comparing two floating points directly: probably not advisable?
  let refresh_frequency = if app.args.refresh_frequency != 0.0 {
    app.args.refresh_frequency
  } else {
    match app.profile {
      Some(profile) => profile.refresh_frequency.unwrap_or(0.0),
      None => 0.0,
    }
  };

  let mut terminal_manager = TerminalManager::new()?;

  let (tx, rx) = mpsc::channel();
  let (loading_tx, loading_rx) = mpsc::channel();

  poll_events(&tx);
  poll_refetches(&tx, refresh_frequency);
  poll_loading(&tx, loading_rx);

  tx.send(Event::RefetchData(false)).unwrap();

  loop {
    terminal_manager
      .terminal
      .draw(|frame| ui::draw(frame, &mut app))?;

    let mut on_event = |event: Event<KeyEvent>| -> Result<bool, Box<dyn Error>> {
      handle_event(
        event,
        &mut app,
        &mut terminal_manager,
        &tx,
        lines_to_skip,
        &loading_tx,
      )
    };

    // You might be wondering, what's going on here? As it so happens, we're blocking until the first event is received, and then processing any other events in the buffer before continuing. If we only handle one event per iteration of the loop, that's a lot of unnecessary drawing. On the other hand, if we don't block on any events, we'll end up drawing constantly while waiting for the next event to be received, causing CPU to go through the roof.
    if !on_event(rx.recv()?)? {
      break;
    }

    for backlogged_event in rx.try_iter() {
      if !on_event(backlogged_event)? {
        break;
      }
    }
  }

  Ok(())
}

fn poll_events(tx: &Sender<Event<KeyEvent>>) {
  let tick_rate = Duration::from_millis(10000); // TODO: do we actually need this?
  let tx_clone = tx.clone();

  thread::spawn(move || {
    let mut last_tick = Instant::now();
    loop {
      // poll for tick rate duration, if no events, sent tick event.
      let timeout = tick_rate
        .checked_sub(last_tick.elapsed())
        .unwrap_or_else(|| Duration::from_secs(0));
      if event::poll(timeout).unwrap() {
        if let CEvent::Key(key) = event::read().unwrap() {
          tx_clone.send(Event::Input(key)).unwrap();
        }
      }

      if last_tick.elapsed() >= tick_rate {
        last_tick = Instant::now();
      }
    }
  });
}

fn poll_refetches(tx: &Sender<Event<KeyEvent>>, refresh_frequency: f64) {
  if refresh_frequency == 0.0 {
    return;
  }

  let tick_rate = Duration::from_millis((refresh_frequency * 1000.0).round() as u64);
  let tx_clone = tx.clone();

  thread::spawn(move || {
    let ticker = Ticker::new(0.., tick_rate);
    for _ in ticker {
      tx_clone.send(Event::RefetchData(true)).ok();
    }
  });
}

fn poll_loading(tx: &Sender<Event<KeyEvent>>, loading_rx: Receiver<bool>) {
  let tx_clone = tx.clone();

  thread::spawn(move || {
    let interval = Duration::from_millis(100);
    let mut is_loading = false;

    loop {
      thread::sleep(interval);

      is_loading = if is_loading {
        match loading_rx.try_recv() {
          Ok(v) => v,
          Err(mpsc::TryRecvError::Empty) => is_loading,
          Err(e) => panic!("Unexpected error: {:?}", e),
        }
      } else {
        match loading_rx.recv() {
          Ok(v) => v,
          // we get this error when we quit the application so we're just returning false for now
          Err(_) => false,
        }
      };

      if is_loading {
        tx_clone.send(Event::Tick).unwrap();
      }
    }
  });
}

fn handle_event(
  event: Event<KeyEvent>,
  app: &mut App,
  terminal_manager: &mut TerminalManager,
  tx: &Sender<Event<KeyEvent>>,
  lines_to_skip: usize,
  loading_tx: &Sender<bool>,
) -> Result<bool, Box<dyn Error>> {
  match event {
    Event::Input(event) => {
      if event.code == KeyCode::Char('c') && event.modifiers == KeyModifiers::CONTROL {
        terminal_manager.teardown()?;
        return Ok(false);
      }

      match app.focused_panel {
        FocusedPanel::Table => match event.code {
          KeyCode::Char('q') => {
            terminal_manager.teardown()?;
            return Ok(false);
          }
          KeyCode::Esc => {
            app.reset_filter_text();
          }
          KeyCode::Down | KeyCode::Char('j') => {
            app.table.next();
            app.on_select();
          }
          KeyCode::Up | KeyCode::Char('k') => {
            app.table.previous();
            app.on_select();
          }
          KeyCode::Char('/') => {
            app.focused_panel = FocusedPanel::Search;
          }
          KeyCode::Char('$') => {
            // TODO: wonder if the typical user would prefer opening the file or switching to vim to edit it? If they do want to open it, we probably need an OS-specific command to be entered here.
            run_command(
              app,
              loading_tx,
              tx,
              format!(
                "{} \"{}\"",
                os_commands::open_command(),
                app.config_path.to_str().unwrap()
              ),
            );
          }
          KeyCode::Char(c) => {
            handle_keybinding_press(app, loading_tx, tx, c);
          }
          _ => (),
        },
        FocusedPanel::Search => match event.code {
          KeyCode::Backspace => {
            app.pop_filter_text_char();
          }
          KeyCode::Esc => {
            app.reset_filter_text();
            app.focused_panel = FocusedPanel::Table;
          }
          KeyCode::Enter => {
            app.focused_panel = FocusedPanel::Table;
          }
          KeyCode::Char(c) => {
            app.push_filter_text_char(c);
          }
          _ => (),
        },
        FocusedPanel::ErrorPopup(_) => match event.code {
          KeyCode::Char('q') => {
            terminal_manager.teardown()?;
            return Ok(false);
          }
          KeyCode::Esc => {
            app.focused_panel = FocusedPanel::Table;
          }
          _ => {}
        },
        FocusedPanel::ConfirmationPopup(ref command) => match event.code {
          KeyCode::Enter => {
            // interesting lesson here: if I have command.clone() in the call to run_command itself (i.e. no intermediate variable) I get an error for borrowing app twice because I borrow it once to get the command and then I pass it as a mutable reference into the run_command function. With this intermediate variable, rust knows we no longer need the reference to app so I'm okay to go ahead and get the mutable reference.
            let cloned_command = command.clone();
            run_command(app, loading_tx, tx, cloned_command);
            app.focused_panel = FocusedPanel::Table;
          }
          KeyCode::Char('q') => {
            terminal_manager.teardown()?;
            return Ok(false);
          }
          KeyCode::Esc => {
            app.focused_panel = FocusedPanel::Table;
          }
          _ => {}
        },
      }
    }

    Event::Tick => {
      app.on_tick();
    }
    Event::RefetchData(background) => {
      refetch_data(app, tx, lines_to_skip, loading_tx, background);
    }
    Event::RowsLoaded(rows) => {
      on_rows_loaded(app, loading_tx, rows);
    }
    Event::Error(error) => {
      app.focused_panel = FocusedPanel::ErrorPopup(error);
      app.status_text = None;
    }
  }

  Ok(true)
}

fn handle_keybinding_press(
  app: &mut App,
  loading_tx: &Sender<bool>,
  tx: &Sender<Event<KeyEvent>>,
  c: char,
) -> Option<()> {
  let binding = app.profile?.key_bindings.iter().find(|&kb| kb.key == c)?;

  let command = template::resolve_command(binding, app.get_selected_row()?);

  if binding.confirm {
    app.focused_panel = FocusedPanel::ConfirmationPopup(command);
  } else {
    run_command(app, loading_tx, tx, command);
  }

  Some(())
}

fn run_command(
  app: &mut App,
  loading_tx: &Sender<bool>,
  tx: &Sender<Event<KeyEvent>>,
  command: String,
) {
  app.status_text = Some(format!("Running command: {}", command));
  loading_tx.send(true).unwrap();

  let tx_clone = tx.clone();
  thread::spawn(move || match command::run_command(&command) {
    Ok(_) => tx_clone.send(Event::RefetchData(false)).unwrap(),
    Err(error) => tx_clone.send(Event::Error(error)).unwrap(),
  });
}

fn refetch_data(
  app: &mut App,
  tx: &Sender<Event<KeyEvent>>,
  lines_to_skip: usize,
  loading_tx: &Sender<bool>,
  background: bool,
) {
  let command = app.args.command.clone();
  app.status_text = Some(if background {
    String::from("")
  } else {
    format!("Running command: {} (if this is taking a while the program might be continuously streaming data which is not yet supported)", command)
  });
  loading_tx.send(true).unwrap();

  let tx_clone = tx.clone();
  thread::spawn(move || {
    let rows = get_rows_from_command(&command, lines_to_skip);

    match rows {
      Ok(rows) => tx_clone.send(Event::RowsLoaded(rows)).unwrap(),
      Err(error) => tx_clone.send(Event::Error(error)).unwrap(),
    }
  });
}

fn get_rows_from_command(command: &str, skip_lines: usize) -> Result<Vec<Row>, String> {
  let output = command::run_command(command)?;

  let trimmed_output = output
    .lines()
    .skip(skip_lines)
    .collect::<Vec<&str>>()
    .join("\n");

  Ok(parse::parse(trimmed_output))
}

fn on_rows_loaded(app: &mut App, loading_tx: &Sender<bool>, rows: Vec<Row>) {
  app.update_rows(rows);

  app.status_text = None;
  loading_tx.send(false).unwrap();
}
