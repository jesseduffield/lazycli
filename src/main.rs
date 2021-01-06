#[allow(dead_code)]
mod args;
mod config;
mod parse;
mod template;
mod util;

use crate::util::event::{Event, Events};
use args::Args;
use config::{Config, Profile};
use std::cmp;
use std::{error::Error, io};
use termion::{event::Key, input::MouseTerminal, raw::IntoRawMode, screen::AlternateScreen};
use tui::{
    backend::TermionBackend,
    layout::{Constraint, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Cell, Paragraph, Row, Table, Wrap},
    Terminal,
};
use util::command;
use util::stateful_table::StatefulTable;

struct App {
    rows: Vec<parse::Row>,
    table: StatefulTable,
}

impl App {
    fn new(rows: Vec<parse::Row>) -> App {
        App {
            table: StatefulTable::new(rows.len()),
            rows: rows,
        }
    }

    fn get_selected_row(&self) -> &parse::Row {
        let selected_index = self.table.state.selected().unwrap();

        &self.rows[selected_index]
    }

    fn update_rows(&mut self, rows: Vec<parse::Row>) {
        let length = rows.len();
        self.table.row_count = length;
        self.rows = rows;
        // if our cursor is too far we need to correct it
        if length == 0 {
            self.table.state.select(Some(0));
        } else if self.table.state.selected().unwrap() > length - 1 {
            self.table.state.select(Some(length - 1));
        }
    }
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

fn get_column_widths(rows: &Vec<parse::Row>) -> std::vec::Vec<tui::layout::Constraint> {
    if rows.len() == 0 {
        return vec![];
    }

    rows.iter()
        .map(|row| row.cells.iter().map(|cell| cell.len()).collect())
        .fold(
            std::iter::repeat(0)
                .take(rows[0].cells.len())
                .collect::<Vec<usize>>(),
            |acc: Vec<usize>, curr: Vec<usize>| {
                acc.into_iter()
                    .zip(curr.into_iter())
                    .map(|(a, b)| cmp::max(a, b))
                    .collect()
            },
        )
        .into_iter()
        .map(|width| Constraint::Length(width as u16))
        .collect::<Vec<Constraint>>()
}

fn load_rows(rows: Vec<parse::Row>) -> std::vec::Vec<std::vec::Vec<String>> {
    rows.into_iter()
        .map(|row| row.cells.iter().map(|cell| cell.to_owned()).collect())
        .collect::<Vec<Vec<String>>>()
}

fn display_keybindings(profile: Option<&Profile>, app: &App) -> String {
    default_keybindings()
        .into_iter()
        .chain(match profile {
            Some(profile) => match profile.key_bindings.len() {
                0 => vec![String::from("No keybindings set")],
                _ => profile
                    .key_bindings
                    .iter()
                    .map(|kb| {
                        format!(
                            "{}: `{}`",
                            kb.key,
                            template::resolve_command(&kb, &app.get_selected_row())
                        )
                    })
                    .collect::<Vec<String>>(),
            },
            None => vec![String::from("No profile selected")],
        })
        .collect::<Vec<String>>()
        .join("\n")
}

fn default_keybindings() -> Vec<String> {
    vec![String::from("▲/▼/j/k: up/down"), String::from("q: quit")]
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::new();

    let config = Config::new();
    let profile = config
        .profiles
        .iter()
        .find(|p| p.registered_commands.iter().any(|c| *c == args.command));

    let lines_to_skip = match args.lines_to_skip {
        0 => match profile {
            Some(profile) => profile.lines_to_skip,
            None => 0,
        },
        _ => args.lines_to_skip,
    };

    // maintain a rows array here and derive raw_rows on each loop? That way we can use selected_index and get the original row itself.
    let mut original_rows = get_rows_from_command(&args.command, lines_to_skip);

    let events = Events::new();

    let mut app = App::new(original_rows);
    app.table.next();

    let mut terminal = prepare_terminal()?;

    let selected_style = Style::default()
        .bg(Color::Blue)
        .add_modifier(Modifier::BOLD);

    // Input
    loop {
        terminal.draw(|f| {
            // need to get bindings for this profile
            let formatted_bindings = display_keybindings(profile, &app);

            let formatted_keybindings_length = (formatted_bindings.lines().count() + 1) as u16;

            let rects = Layout::default()
                .constraints(
                    [
                        Constraint::Length(f.size().height - formatted_keybindings_length),
                        Constraint::Length(formatted_keybindings_length),
                    ]
                    .as_ref(),
                )
                .split(f.size());

            let rows = app.rows.iter().map(|row| {
                let cells = row.cells.iter().map(|c| Cell::from(c.clone()));
                Row::new(cells).height(1)
            });

            let widths = get_column_widths(&app.rows);

            let table = Table::new(rows)
                // .block(Block::default().borders(Borders::ALL).title("Table"))
                .highlight_style(selected_style)
                .highlight_symbol("> ")
                .widths(&widths)
                .column_spacing(2);

            f.render_stateful_widget(table, rects[0], &mut app.table.state);

            let keybindings_list = Paragraph::new(formatted_bindings)
                .block(Block::default().title(match profile {
                    Some(profile) => format!("Keybindings for profile '{}':", profile.name),
                    None => String::from("Keybindings:"),
                }))
                .style(Style::default().fg(Color::Reset))
                .wrap(Wrap { trim: true });

            f.render_widget(keybindings_list, rects[1]);
        })?;

        if let Event::Input(key) = events.next()? {
            match key {
                Key::Char('q') => {
                    break;
                }
                Key::Down | Key::Char('k') => {
                    app.table.next();
                }
                Key::Up | Key::Char('j') => {
                    app.table.previous();
                }
                Key::Char(c) => {
                    if profile.is_some() {
                        let binding = profile.unwrap().key_bindings.iter().find(|&kb| kb.key == c);
                        if binding.is_some() {
                            let command = template::resolve_command(
                                &binding.unwrap(),
                                &app.get_selected_row(),
                            );
                            let output = command::run_command(&command).unwrap();
                            original_rows = get_rows_from_command(&args.command, lines_to_skip);
                            app.update_rows(original_rows);
                        }
                    }
                }
                _ => (),
            }
        };
    }

    Ok(())
}
