use std::io;
use std::process::Command;

pub fn run_command(command: &str) -> io::Result<String> {
  let output = Command::new("sh").args(&["-c", command]).output()?;

  Ok(String::from_utf8(output.stdout).unwrap())
}

#[cfg(test)]
#[test]
fn test_run_command() {
  let result = run_command("echo 1").unwrap();
  assert_eq!(result, "1\n");
}
