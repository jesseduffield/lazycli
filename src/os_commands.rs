#[cfg(target_os = "linux")]
pub fn open_command() -> String {
  String::from("xdg-open")
}

#[cfg(target_os = "macos")]
pub fn open_command() -> String {
  String::from("open")
}

#[cfg(target_os = "windows")]
pub fn open_command() -> String {
  String::from("start")
}
