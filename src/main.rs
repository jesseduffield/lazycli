#[allow(dead_code)]
mod args;
mod parse;
mod util;

use crate::util::event::{Event, Events};
use args::Args;
use std::cmp;
use std::{error::Error, io};
use termion::{event::Key, input::MouseTerminal, raw::IntoRawMode, screen::AlternateScreen};
use tui::{
    backend::TermionBackend,
    layout::{Constraint, Layout},
    style::{Color, Modifier, Style},
    widgets::{Cell, Row, Table},
    Terminal,
};
use util::command;
use util::stateful_table::StatefulTable;

struct App<'a> {
    table: StatefulTable<'a>,
}

impl<'a> App<'a> {
    fn new(rows: Vec<Vec<&'a str>>) -> App<'a> {
        App {
            table: StatefulTable::new(rows),
        }
    }
}

fn get_rows_from_command(command: &str, skip_lines: usize) -> Vec<parse::Row> {
    let output = command::run_command(command).unwrap();

    println!("{:?}", output);

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

fn get_column_widths(app: &App) -> std::vec::Vec<tui::layout::Constraint> {
    if app.table.rows.len() == 0 {
        return vec![];
    }

    app.table
        .rows
        .iter()
        .map(|row| row.iter().map(|cell| cell.len()).collect())
        .fold(
            std::iter::repeat(0)
                .take(app.table.rows[0].len())
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

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::new();

    let raw_rows = get_rows_from_command(&args.command, args.lines_to_skip)
        .into_iter()
        .map(|row| row.cells.iter().map(|cell| cell.to_owned()).collect())
        .collect::<Vec<Vec<String>>>();

    let raw_rows_as_strs = raw_rows
        .iter()
        .map(|row| row.iter().map(|cell| cell.as_str()).collect())
        .collect::<Vec<Vec<&str>>>();

    let mut terminal = prepare_terminal()?;

    let events = Events::new();

    let mut app = App::new(raw_rows_as_strs);
    app.table.next();

    // Input
    loop {
        terminal.draw(|f| {
            let rects = Layout::default()
                .constraints([Constraint::Percentage(100)].as_ref())
                .split(f.size());

            let selected_style = Style::default()
                .bg(Color::Blue)
                .add_modifier(Modifier::BOLD);

            let rows = app.table.rows.iter().map(|item| {
                let cells = item.iter().map(|c| Cell::from(*c));
                Row::new(cells).height(1)
            });

            let widths = get_column_widths(&app);

            let t = Table::new(rows)
                // .block(Block::default().borders(Borders::ALL).title("Table"))
                .highlight_style(selected_style)
                .highlight_symbol("> ")
                .widths(&widths)
                .column_spacing(2);
            f.render_stateful_widget(t, rects[0], &mut app.table.state);
        })?;

        if let Event::Input(key) = events.next()? {
            match key {
                Key::Char('q') => {
                    break;
                }
                Key::Down => {
                    app.table.next();
                }
                Key::Up => {
                    app.table.previous();
                }
                _ => {}
            }
        };
    }

    Ok(())
}
