// adapted from https://stackoverflow.com/questions/53974404/replacing-numbered-placeholders-with-elements-of-a-vector-in-rust

use regex::{Captures, Regex};

use crate::config::Command;
use crate::parse::Row;

pub fn resolve_command(command: &dyn Command, row: &Row) -> String {
  // if keybinding has a regex we need to use that, otherwise we generate the regex ourselves
  let matches = match &command.regex() {
    Some(regex) => {
      let regex = Regex::new(regex).unwrap(); // TODO: handle malformed regex
      match regex.captures(&row.original_line) {
        None => vec![],
        Some(captures) => captures
          .iter()
          .map(|capture| match capture {
            Some(capture) => capture.as_str(),
            None => "",
          })
          .collect::<Vec<&str>>(),
      }
    }
    None => row.cells_as_strs(),
  };

  template_replace(&command.command(), &matches)
}

pub fn template_replace(template: &str, values: &[&str]) -> String {
  let regex = Regex::new(r#"\$(\d+)"#).unwrap();
  regex
    .replace_all(template, |captures: &Captures| {
      values.get(index(captures)).unwrap_or(&"")
    })
    .to_string()
}

fn index(captures: &Captures) -> usize {
  captures.get(1).unwrap().as_str().parse().unwrap()
}
