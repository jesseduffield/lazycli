// adapted from https://stackoverflow.com/questions/53974404/replacing-numbered-placeholders-with-elements-of-a-vector-in-rust

use regex::{Captures, Regex};

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
