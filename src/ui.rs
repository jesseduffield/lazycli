use crate::app::App;
use crate::config::Profile;
use crate::parse;
use crate::template;
use std::cmp;
use std::time::SystemTime;
use tui::{
  backend::Backend,
  layout::{Constraint, Direction, Layout, Rect},
  style::{Color, Modifier, Style},
  widgets::{Block, Cell, Paragraph, Row, Table, Wrap},
  Frame,
};

static SPINNER_STATES: &[char] = &['⣾', '⣷', '⣯', '⣟', '⡿', '⢿', '⣻', '⣽'];

fn spinner_frame() -> String {
  // need current time
  let now = SystemTime::now()
    .duration_since(SystemTime::UNIX_EPOCH)
    .unwrap()
    .as_millis()
    / 100;

  let index = (now as usize) % (SPINNER_STATES.len() - 1);
  SPINNER_STATES[index].to_string()
}

pub fn draw<B: Backend>(frame: &mut Frame<B>, app: &mut App) {
  let formatted_bindings = display_keybindings(app.profile, &app);
  let formatted_keybindings_height = (formatted_bindings.lines().count() + 1) as u16;

  let rects = Layout::default()
    .constraints(
      [
        Constraint::Length(frame.size().height - 1),
        Constraint::Length(1),
      ]
      .as_ref(),
    )
    .split(frame.size());

  draw_status_bar(app, rects[1], frame);

  {
    let rects = Layout::default()
      .direction(Direction::Horizontal)
      //  no right panel for now so setting that to 0 percent
      .constraints([Constraint::Percentage(100), Constraint::Percentage(0)].as_ref())
      .split(rects[0]);

    draw_item_render(app, rects[1], frame);

    {
      let rects = Layout::default()
        .constraints([
          Constraint::Length(rects[0].height - formatted_keybindings_height - 2),
          Constraint::Length(1),
          Constraint::Length(formatted_keybindings_height),
        ])
        .split(rects[0]);

      draw_table(app, rects[0], frame);
      draw_keybindings(app, rects[2], frame, formatted_bindings);
    }
  }
}

fn draw_table<B: Backend>(app: &mut App, rect: Rect, frame: &mut tui::Frame<B>) {
  let selected_style = Style::default()
    .bg(Color::Blue)
    .add_modifier(Modifier::BOLD);

  let filtered_rows = app.filtered_rows();
  let rows = filtered_rows.iter().map(|row| {
    let cells = row.cells.iter().map(|c| Cell::from(c.clone()));
    Row::new(cells).height(1)
  });

  let widths = get_column_widths(&filtered_rows);

  let table = Table::new(rows)
    .highlight_style(selected_style)
    .highlight_symbol("> ")
    .widths(&widths)
    .column_spacing(2);

  frame.render_stateful_widget(table, rect, &mut app.table.state);
}

fn draw_keybindings<B: Backend>(
  app: &mut App,
  rect: Rect,
  frame: &mut tui::Frame<B>,
  formatted_bindings: String,
) {
  let keybindings_list = Paragraph::new(formatted_bindings)
    .block(Block::default().title(match app.profile {
      Some(profile) => format!("Keybindings for profile '{}':", profile.name),
      None => String::from("Keybindings:"),
    }))
    .style(Style::default().fg(Color::Reset))
    .wrap(Wrap { trim: true });

  frame.render_widget(keybindings_list, rect);
}

fn draw_status_bar<B: Backend>(app: &mut App, rect: Rect, frame: &mut tui::Frame<B>) {
  let status_text = match app.status_text.as_ref() {
    Some(text) => match text {
      _ => format!("{} {}", spinner_frame(), text),
    },
    None => String::from(""),
  };

  let status_bar = Paragraph::new(status_text).style(Style::default().fg(Color::Cyan));

  frame.render_widget(status_bar, rect);
}

fn draw_item_render<B: Backend>(app: &mut App, rect: Rect, frame: &mut tui::Frame<B>) {
  let paragraph = Paragraph::new("TEST").style(Style::default().fg(Color::Cyan));

  frame.render_widget(paragraph, rect);
}

fn get_column_widths(rows: &Vec<&parse::Row>) -> std::vec::Vec<tui::layout::Constraint> {
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
