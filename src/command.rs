use std::process::Command;

pub fn run_command(command: &str) -> Result<String, String> {
  let output = Command::new("bash")
    .args(&["-c", command])
    .output()
    .expect(&format!("failed to run command {}", command));

  if !output.status.success() {
    return Err(String::from_utf8(output.stderr).unwrap());
  }

  Ok(String::from_utf8(output.stdout).unwrap())
}

#[cfg(test)]
#[test]
fn test_run_command() {
  let result = run_command("echo 1");
  assert_eq!(result, Ok(String::from("1\n")));
}

#[test]
fn test_run_command_fail() {
  let result = run_command("asldfkjh test");
  assert_eq!(
    result,
    Err(String::from("bash: asldfkjh: command not found\n"))
  );
}
