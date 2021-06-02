#[cfg(target_os = "linux")]
pub fn open_command(path: &str) -> String {
  format!("xdg-open \"{}\"", path)
}

#[cfg(target_os = "macos")]
pub fn open_command(path: &str) -> String {
  format!("open \"{}\"", path)
}

#[cfg(target_os = "windows")]
pub fn open_command(path: &str) -> String {
  format!("start \"\" \"{}\"", path)
}
