use crate::app::App;
use crate::config::Profile;
use crate::parse;
use crate::template;
use std::cmp;
use tui::{
  backend::Backend,
  layout::{Constraint, Layout},
  style::{Color, Modifier, Style},
  widgets::{Block, Cell, Paragraph, Row, Table, Wrap},
  Frame,
};

pub fn draw<B: Backend>(f: &mut Frame<B>, app: &mut App) {
  let selected_style = Style::default()
    .bg(Color::Blue)
    .add_modifier(Modifier::BOLD);

  // need to get bindings for this profile
  let formatted_bindings = display_keybindings(app.profile, &app);

  let formatted_keybindings_length = (formatted_bindings.lines().count() + 1) as u16;

  let rects = Layout::default()
    .constraints(
      [
        Constraint::Length(f.size().height - formatted_keybindings_length - 1),
        Constraint::Length(formatted_keybindings_length),
        Constraint::Length(1),
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
    .highlight_style(selected_style)
    .highlight_symbol("> ")
    .widths(&widths)
    .column_spacing(2);

  f.render_stateful_widget(table, rects[0], &mut app.table.state);

  let keybindings_list = Paragraph::new(formatted_bindings)
    .block(Block::default().title(match app.profile {
      Some(profile) => format!("Keybindings for profile '{}':", profile.name),
      None => String::from("Keybindings:"),
    }))
    .style(Style::default().fg(Color::Reset))
    .wrap(Wrap { trim: true });

  f.render_widget(keybindings_list, rects[1]);

  let status_bar = Paragraph::new(if app.is_loading { "Loading..." } else { "" })
    .style(Style::default().fg(Color::Cyan));

  f.render_widget(status_bar, rects[2]);
}

fn get_column_widths(rows: &Vec<parse::Row>) -> std::vec::Vec<tui::layout::Constraint> {
  if rows.len() == 0 {
    return vec![];
  }

  rows
    .iter()
    .map(|row| row.cells.iter().map(|cell| cell.len()).collect())
    .fold(
      std::iter::repeat(0)
        .take(rows[0].cells.len())
        .collect::<Vec<usize>>(),
      |acc: Vec<usize>, curr: Vec<usize>| {
        acc
          .into_iter()
          .zip(curr.into_iter())
          .map(|(a, b)| cmp::max(a, b))
          .collect()
      },
    )
    .into_iter()
    .map(|width| Constraint::Length(width as u16))
    .collect::<Vec<Constraint>>()
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
              "{}: {}",
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
