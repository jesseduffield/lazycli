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
use config::{Config, Profile};

use std::io::Read;
use std::{error::Error, io};
use termion::{
    async_stdin, event::Key, input::MouseTerminal, input::TermRead, raw::IntoRawMode,
    screen::AlternateScreen,
};
use tui::{
    backend::TermionBackend,
    layout::{Constraint, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Cell, Paragraph, Row, Table, Wrap},
    Terminal,
};
use util::command;

// crossterm stuff

// use crate::demo::{ui, App};
// use argh::FromArgs;
// use crossterm::{
//     event::{self, DisableMouseCapture, EnableMouseCapture, Event as CEvent, KeyCode},
//     execute,
//     terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
// };
// use std::{
//     error::Error,
//     io::stdout,
//     sync::mpsc,
//     thread,
//     time::{Duration, Instant},
// };
// use tui::{backend::CrosstermBackend, Terminal};

// enum Event<I> {
//     Input(I),
//     Tick,
// }

// /// Crossterm demo
// #[derive(Debug, FromArgs)]
// struct Cli {
//     /// time in ms between two ticks.
//     #[argh(option, default = "250")]
//     tick_rate: u64,
//     /// whether unicode symbols are used to improve the overall look of the app
//     #[argh(option, default = "true")]
//     enhanced_graphics: bool,
// }

fn get_rows_from_command(command: &str, skip_lines: usize) -> Vec<parse::Row> {
    let output = command::run_command(command).unwrap();

    let trimmed_output = output
        .lines()
        .skip(skip_lines)
        .collect::<Vec<&str>>()
        .join("\n");

    parse::parse(trimmed_output)
}

fn prepare_terminal() -> Result<
    tui::Terminal<
        tui::backend::TermionBackend<
            termion::screen::AlternateScreen<
                termion::input::MouseTerminal<termion::raw::RawTerminal<std::io::Stdout>>,
            >,
        >,
    >,
    std::io::Error,
> {
    // Terminal initialization
    let stdout = io::stdout().into_raw_mode()?;
    let stdout = MouseTerminal::from(stdout);
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    Terminal::new(backend)
}

// fn main() -> Result<(), Box<dyn Error>> {
//     let cli: Cli = argh::from_env();

//     enable_raw_mode()?;

//     let mut stdout = stdout();
//     execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

//     let backend = CrosstermBackend::new(stdout);

//     let mut terminal = Terminal::new(backend)?;

//     // Setup input handling
//     let (tx, rx) = mpsc::channel();

//     let tick_rate = Duration::from_millis(cli.tick_rate);
//     thread::spawn(move || {
//         let mut last_tick = Instant::now();
//         loop {
//             // poll for tick rate duration, if no events, sent tick event.
//             let timeout = tick_rate
//                 .checked_sub(last_tick.elapsed())
//                 .unwrap_or_else(|| Duration::from_secs(0));
//             if event::poll(timeout).unwrap() {
//                 if let CEvent::Key(key) = event::read().unwrap() {
//                     tx.send(Event::Input(key)).unwrap();
//                 }
//             }
//             if last_tick.elapsed() >= tick_rate {
//                 tx.send(Event::Tick).unwrap();
//                 last_tick = Instant::now();
//             }
//         }
//     });

//     let mut app = App::new("Crossterm Demo", cli.enhanced_graphics);

//     terminal.clear()?;

//     loop {
//         terminal.draw(|f| ui::draw(f, &mut app))?;
//         match rx.recv()? {
//             Event::Input(event) => match event.code {
//                 KeyCode::Char('q') => {
//                     disable_raw_mode()?;
//                     execute!(
//                         terminal.backend_mut(),
//                         LeaveAlternateScreen,
//                         DisableMouseCapture
//                     )?;
//                     terminal.show_cursor()?;
//                     break;
//                 }
//                 KeyCode::Char(c) => app.on_key(c),
//                 KeyCode::Left => app.on_left(),
//                 KeyCode::Up => app.on_up(),
//                 KeyCode::Right => app.on_right(),
//                 KeyCode::Down => app.on_down(),
//                 _ => {}
//             },
//             Event::Tick => {
//                 app.on_tick();
//             }
//         }
//         if app.should_quit {
//             break;
//         }
//     }

//     Ok(())
// }

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
    let mut original_rows = get_rows_from_command(&app.args.command, lines_to_skip);

    app.update_rows(original_rows);

    let mut terminal = prepare_terminal()?;

    // Create a separate thread to poll stdin.
    // This provides non-blocking input support.
    let mut asi = async_stdin();

    // Input
    loop {
        terminal.draw(|f| ui::draw(f, &mut app))?;

        // Iterate over all the keys that have been pressed since the
        // last time we checked.
        // need to wait for event
        for k in asi.by_ref().keys() {
            match k.unwrap() {
                Key::Char('q') => {
                    terminal.clear()?;
                    return Ok(());
                }
                Key::Down | Key::Char('k') => {
                    app.table.next();
                }
                Key::Up | Key::Char('j') => {
                    app.table.previous();
                }
                Key::Char(c) => {
                    if app.profile.is_some() {
                        let binding = app
                            .profile
                            .unwrap()
                            .key_bindings
                            .iter()
                            .find(|&kb| kb.key == c);
                        if binding.is_some() {
                            let command = template::resolve_command(
                                &binding.unwrap(),
                                &app.get_selected_row(),
                            );
                            command::run_command(&command)?;
                            // need to set the app state here, then run the command asynchronously and once it's done, update the app.
                            original_rows = get_rows_from_command(&app.args.command, lines_to_skip);
                            app.update_rows(original_rows);
                        }
                    }
                }
                _ => (),
            }
        }
    }
}
