use regex::Regex;

// this function tells us whether the entered command matches a given command pattern
// associated with a profile of keybindings
pub fn command_matches(command: &str, pattern: &str) -> bool {
  let regex_str_inner = pattern
    .split("*")
    .map(|chunk| regex::escape(chunk))
    .collect::<Vec<String>>()
    .join(".*");

  let regex_str = ["^", regex_str_inner.as_ref(), "$"].join("");

  let re = Regex::new(regex_str.as_ref()).unwrap();

  re.is_match(command)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_command_matches_exact_match() {
    assert!(
      command_matches("ls", "ls"),
      "should have returned true for exact match on pattern 'ls'",
    )
  }

  #[test]
  fn test_command_matches_non_exact_match() {
    assert!(
      !command_matches("ls", "ls blah"),
      "should have returned false for mismatch",
    )
  }

  #[test]
  fn test_command_matches_with_wildcard() {
    assert!(
      command_matches("ls -l", "ls *"),
      "should have matched due to wildcard",
    )
  }

  #[test]
  fn test_command_does_not_match_with_wildcard() {
    assert!(
      !command_matches("git status", "ls *"),
      "should not have matched despite wildcard",
    )
  }

  #[test]
  fn test_with_escaped_characters() {
    assert!(
      !command_matches("blah", "(blah())/.txt*.blah"),
      "should not have matched despite wildcard",
    )
  }

  #[test]
  fn test_with_escaped_characters_when_matched() {
    assert!(
      command_matches("ls blah/blah.txt", "ls *"),
      "should have matched",
    )
  }

  #[test]
  fn test_with_multiple_wildcards() {
    assert!(
      command_matches("blah hehe haha hmm lol", "blah*haha*lol"),
      "should not have matched due to multiple wildcards",
    )
  }
}
