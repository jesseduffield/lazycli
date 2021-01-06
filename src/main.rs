#[allow(dead_code)]
mod app;
mod args;
mod config;
mod parse;
mod template;
mod ui;
mod util;

use app::App;
use args::Args;
use config::Config;

use std::error::Error;

use tui::{backend::CrosstermBackend, Terminal};
use util::command;

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event as CEvent, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{
    io::stdout,
    sync::mpsc,
    thread,
    time::{Duration, Instant},
};

enum Event<I> {
    Input(I),
    Tick,
    CommandFinished,
}

fn get_rows_from_command(command: &str, skip_lines: usize) -> Vec<parse::Row> {
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

    // maintain a rows array here and derive raw_rows on each loop? That way we can use selected_index and get the original row itself.
    let original_rows = get_rows_from_command(&app.args.command, lines_to_skip);

    app.update_rows(original_rows);

    let mut terminal_manager = TerminalManager::new()?;

    // Setup input handling
    let (tx, rx) = mpsc::channel();
    let tx_clone = tx.clone();

    let tick_rate = Duration::from_millis(1000); // TODO: consider changing value
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

    terminal_manager.terminal.clear()?;

    loop {
        terminal_manager.terminal.draw(|f| ui::draw(f, &mut app))?;
        match rx.recv()? {
            Event::Input(event) => match event.code {
                KeyCode::Char('q') => {
                    terminal_manager.teardown()?;
                    break;
                }
                KeyCode::Down | KeyCode::Char('k') => {
                    app.table.next();
                }
                KeyCode::Up | KeyCode::Char('j') => {
                    app.table.previous();
                }
                KeyCode::Char(c) => {
                    if app.profile.is_some() {
                        let binding = app
                            .profile
                            .unwrap()
                            .key_bindings
                            .iter()
                            .find(|&kb| kb.key == c);

                        if binding.is_some() {
                            app.is_loading = true;

                            let command = template::resolve_command(
                                &binding.unwrap(),
                                app.get_selected_row(),
                            );

                            let tx_clone = tx.clone();

                            thread::spawn(move || {
                                // TODO: don't just unwrap here
                                command::run_command(&command).unwrap();

                                // // need to set the app state here, then run the command asynchronously and once it's done, update the app.
                                // let original_rows =
                                //     get_rows_from_command(&app.args.command, lines_to_skip);
                                // app.update_rows(original_rows);

                                // app.is_loading = false;

                                tx_clone.send(Event::CommandFinished).unwrap()
                            });
                        }
                    }
                }
                _ => (),
            },
            Event::Tick => {
                app.on_tick();
            }
            Event::CommandFinished => (),
        }
        if app.should_quit {
            break;
        }
    }

    Ok(())
}

struct TerminalManager {
    terminal: tui::Terminal<tui::backend::CrosstermBackend<std::io::Stdout>>,
}

impl TerminalManager {
    fn new() -> Result<TerminalManager, Box<dyn Error>> {
        enable_raw_mode()?;
        let mut stdout = stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
        let backend = CrosstermBackend::new(stdout);
        Ok(TerminalManager {
            terminal: Terminal::new(backend)?,
        })
    }

    fn teardown(&mut self) -> Result<(), Box<dyn Error>> {
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
