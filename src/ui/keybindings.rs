use crate::app::{App, FocusedPanel};
use crate::config::Profile;
use crate::template;

pub fn display_keybindings(profile: Option<&Profile>, app: &App) -> String {
  let panel_keybindings = match app.focused_panel {
    FocusedPanel::Table => {
      let mut keybindings = vec![
        String::from("▲/▼/j/k: up/down"),
        String::from("/: filter"),
        String::from("esc: clear filter"),
        String::from("q: quit"),
      ];
      keybindings.extend(profile_keybindings(profile, app));
      keybindings
    }
    FocusedPanel::Search => vec![
      String::from("enter: apply filter"),
      String::from("esc: cancel filter"),
    ],
    FocusedPanel::ErrorPopup(_) => vec![String::from("esc: close popup"), String::from("q: quit")],
    FocusedPanel::ConfirmationPopup(_) => vec![
      String::from("enter: run command"),
      String::from("esc: cancel"),
      String::from("q: quit"),
    ],
  };

  panel_keybindings
    .into_iter()
    .collect::<Vec<String>>()
    .join("\n")
}

fn profile_keybindings(profile: Option<&Profile>, app: &App) -> Vec<String> {
  match profile {
    Some(profile) => match profile.key_bindings.len() {
      0 => vec![String::from("No keybindings set")],
      _ => match app.get_selected_row() {
        Some(row) => profile
          .key_bindings
          .iter()
          .map(|kb| format!("{}: {}", kb.key, template::resolve_command(kb, &row)))
          .collect::<Vec<String>>(),
        None => {
          vec![String::from("No item selected")]
        }
      },
    },
    None => vec![String::from("No profile selected")],
  }
}
