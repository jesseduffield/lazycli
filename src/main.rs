#[allow(dead_code)]
mod app;
mod args;
mod config;
mod parse;
mod template;
mod terminal_manager;
mod ui;
mod util;

use app::{App, FocusedPanel};
use args::Args;
use config::Config;
use parse::Row;

use std::error::Error;

use util::command;

use terminal_manager::TerminalManager;

use crossterm::event::{self, Event as CEvent, KeyCode};
use std::{
    sync::mpsc,
    thread,
    time::{Duration, Instant},
};

enum Event<I> {
    Input(I),
    Tick,
    CommandFinished,
    RowsLoaded(Vec<Row>),
}

fn get_rows_from_command(command: &str, skip_lines: usize) -> Vec<Row> {
    let output = command::run_command(command).unwrap();

    let trimmed_output = output
        .lines()
        .skip(skip_lines)
        .collect::<Vec<&str>>()
        .join("\n");

    parse::parse(trimmed_output)
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::new();
    let config = Config::new();

    let mut app = App::new(&config, args);
    app.table.next();

    let lines_to_skip = match app.args.lines_to_skip {
        0 => match app.profile {
            Some(profile) => profile.lines_to_skip,
            None => 0,
        },
        _ => app.args.lines_to_skip,
    };

    let original_rows = get_rows_from_command(&app.args.command, lines_to_skip);
    app.update_rows(original_rows);

    let mut terminal_manager = TerminalManager::new()?;

    // Setup input handling
    let (tx, rx) = mpsc::channel();
    let tx_clone = tx.clone();

    let tick_rate = Duration::from_millis(10000); // TODO: do we really need this?
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
                tx_clone.send(Event::Tick).unwrap();
                last_tick = Instant::now();
            }
        }
    });

    let tx_clone = tx.clone();
    let (ticker_tx, ticker_rx) = mpsc::channel();
    thread::spawn(move || {
        let interval = Duration::from_millis(100);
        let mut is_loading = false;

        loop {
            thread::sleep(interval);

            is_loading = if is_loading {
                match ticker_rx.try_recv() {
                    Ok(v) => v,
                    Err(mpsc::TryRecvError::Empty) => is_loading,
                    Err(e) => panic!("Unexpected error: {:?}", e),
                }
            } else {
                match ticker_rx.recv() {
                    Ok(v) => v,
                    Err(e) => panic!("Unexpected error: {:?}", e),
                }
            };

            if is_loading {
                tx_clone.send(Event::Tick).unwrap();
            }
        }
    });

    terminal_manager.terminal.clear()?;

    loop {
        terminal_manager
            .terminal
            .draw(|frame| ui::draw(frame, &mut app))?;

        // You might be wondering, what's going on here? As it so happens, we're blocking until the first event is received, and then processing any other events in the buffer before continuing. If we only handle one event per iteration of the loop, that's a lot of unnecessary drawing. On the other hand, if we don't block on any events, we'll end up drawing constantly while waiting for the next event to be received, causing CPU to go through the roof.
        handle_event(
            rx.recv()?,
            &mut app,
            &mut terminal_manager,
            &tx,
            lines_to_skip,
            &ticker_tx,
        )?;
        if app.should_quit {
            break;
        }

        for backlogged_event in rx.try_iter() {
            handle_event(
                backlogged_event,
                &mut app,
                &mut terminal_manager,
                &tx,
                lines_to_skip,
                &ticker_tx,
            )?;
            if app.should_quit {
                break;
            }
        }
    }

    Ok(())
}

fn handle_event(
    event: Event<crossterm::event::KeyEvent>,
    app: &mut App,
    terminal_manager: &mut TerminalManager,
    tx: &mpsc::Sender<Event<crossterm::event::KeyEvent>>,
    lines_to_skip: usize,
    ticker_tx: &mpsc::Sender<bool>,
) -> Result<(), Box<dyn Error>> {
    match event {
        Event::Input(event) => match app.focused_panel {
            FocusedPanel::Table => match event.code {
                KeyCode::Char('q') => {
                    terminal_manager.teardown()?;
                    app.should_quit = true;
                }
                KeyCode::Esc => {
                    app.reset_filter_text();
                }
                KeyCode::Down | KeyCode::Char('k') => {
                    app.table.next();
                }
                KeyCode::Up | KeyCode::Char('j') => {
                    app.table.previous();
                }
                KeyCode::Char('/') => {
                    app.focused_panel = FocusedPanel::Search;
                }
                KeyCode::Char(c) => {
                    match app.get_selected_row() {
                        Some(selected_row) => {
                            if app.profile.is_some() {
                                let binding = app
                                    .profile
                                    .unwrap()
                                    .key_bindings
                                    .iter()
                                    .find(|&kb| kb.key == c);

                                if binding.is_some() {
                                    let command =
                                        template::resolve_command(&binding.unwrap(), selected_row);

                                    app.status_text = Some(format!("Running command: {}", command));
                                    ticker_tx.send(true).unwrap();

                                    let tx_clone = tx.clone();
                                    thread::spawn(move || {
                                        // TODO: don't just unwrap here
                                        command::run_command(&command).unwrap();

                                        tx_clone.send(Event::CommandFinished).unwrap()
                                    });
                                }
                            }
                        }
                        None => (),
                    }
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
        },
        Event::Tick => {
            app.on_tick();
        }
        Event::CommandFinished => {
            let command = app.args.command.clone();
            app.status_text = Some(format!("Running command: {}", command));
            ticker_tx.send(true).unwrap();

            let tx_clone = tx.clone();
            thread::spawn(move || {
                let rows = get_rows_from_command(&command, lines_to_skip);

                tx_clone.send(Event::RowsLoaded(rows)).unwrap()
            });
        }
        Event::RowsLoaded(rows) => {
            app.update_rows(rows);

            app.status_text = None;
            ticker_tx.send(false).unwrap();
        }
    }

    Ok(())
}
