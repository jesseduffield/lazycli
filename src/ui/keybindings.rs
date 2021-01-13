use crate::app::{App, FocusedPanel};
use crate::config::Profile;
use crate::template;

// TODO: derive keybinding menu from our actual key handlers in event_loop.rs
pub fn display_keybindings(profile: Option<&Profile>, app: &App) -> String {
  let panel_keybindings = match app.focused_panel {
    FocusedPanel::Table => {
      let mut keybindings = vec![format!(
        "▲/▼/j/k: navigate, /: filter, esc: clear filter, q: quit, $: open config file (open {})",
        app.config_path.to_str().unwrap()
      )];

      keybindings.extend(profile_keybindings(profile, app));
      keybindings
    }
    FocusedPanel::Search => vec![String::from("enter: apply filter, esc: cancel filter")],
    FocusedPanel::ErrorPopup(_) => vec![String::from("esc: close popup, q: quit")],
    FocusedPanel::ConfirmationPopup(_) => {
      vec![String::from("enter: run command, esc: cancel, q: quit")]
    }
  };

  panel_keybindings
    .into_iter()
    .collect::<Vec<String>>()
    .join("\n")
}

fn profile_keybindings(profile: Option<&Profile>, app: &App) -> Vec<String> {
  match profile {
    Some(profile) => match profile.key_bindings.len() {
      0 => vec![format!("No keybindings set for profile '{}'", profile.name)],
      _ => match app.get_selected_row() {
        Some(row) => {
          let mut result = vec![format!("Keybindings for profile '{}':", profile.name)];

          result.extend(
            profile
              .key_bindings
              .iter()
              .map(|kb| format!("{}: {}", kb.key, template::resolve_command(kb, &row)))
              .collect::<Vec<String>>(),
          );

          result
        }
        None => {
          vec![String::from("No item selected")]
        }
      },
    },
    None => vec![String::from("No profile selected")],
  }
}
