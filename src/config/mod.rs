pub mod prepare;

use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Config {
  pub profiles: Vec<Profile>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Profile {
  pub name: String,
  pub registered_commands: Vec<String>,
  pub key_bindings: Vec<KeyBinding>,
  pub lines_to_skip: usize,
  pub display_command: Option<DisplayCommand>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct KeyBinding {
  pub key: char,
  pub command: String,
  pub confirm: bool,
  pub regex: Option<String>,
}

impl Default for KeyBinding {
  fn default() -> KeyBinding {
    KeyBinding {
      key: ' ',
      command: String::from(""),
      confirm: false,
      regex: None,
    }
  }
}

pub trait Command {
  fn command(&self) -> &str;
  fn regex(&self) -> Option<&str>;
}

// TODO: is there a better way to do this?
impl Command for KeyBinding {
  fn command(&self) -> &str {
    return &self.command;
  }
  fn regex(&self) -> Option<&str> {
    return self.regex.as_deref();
  }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DisplayCommand {
  pub command: String,
  pub regex: Option<String>,
}

// TODO: is there a better way to do this?
impl Command for DisplayCommand {
  fn command(&self) -> &str {
    return &self.command;
  }
  fn regex(&self) -> Option<&str> {
    return self.regex.as_deref();
  }
}

impl Config {
  pub fn to_yaml(&self) -> Result<String, serde_yaml::Error> {
    serde_yaml::to_string(self)
  }

  pub fn from_yaml(yaml: String) -> Result<Config, serde_yaml::Error> {
    serde_yaml::from_str(&yaml)
  }

  pub fn new() -> Config {
    // just doing a dummy one for now
    Config {
      profiles: vec![
        Profile {
          name: String::from("ls"),
          registered_commands: vec![
            String::from("ls -1"),
            String::from("ls -a"),
            String::from("ls"),
          ],
          key_bindings: vec![
            KeyBinding {
              key: 'd',
              command: String::from("rm $0"),
              confirm: true,
              ..Default::default()
            },
            KeyBinding {
              key: 'o',
              command: String::from("code -r $0"),
              ..Default::default()
            },
            KeyBinding {
              key: 'u',
              command: String::from("cd $0"),
              ..Default::default()
            },
          ],
          lines_to_skip: 0,
          // display_command: Some(DisplayCommand {
          //   command: String::from("cat $0"),
          //   regex: None,
          // }),
          display_command: None,
        },
        Profile {
          name: String::from("ls -l"),
          registered_commands: vec![String::from("ls -l")],
          key_bindings: vec![
            KeyBinding {
              key: 'd',
              command: String::from("rm $8"),
              confirm: true,
              ..Default::default()
            },
            KeyBinding {
              key: 'o',
              command: String::from("code -r $8"),
              ..Default::default()
            },
            KeyBinding {
              key: 'u',
              command: String::from("cd $8"),
              ..Default::default()
            },
          ],
          lines_to_skip: 1,
          display_command: None,
        },
        Profile {
          name: String::from("git status --short"),
          registered_commands: vec![String::from("git status --short")],
          key_bindings: vec![
            KeyBinding {
              key: 'A',
              command: String::from("git add $1"),
              ..Default::default()
            },
            KeyBinding {
              key: 'a',
              command: String::from("git reset $1"),
              confirm: true,
              ..Default::default()
            },
            KeyBinding {
              key: 'd',
              command: String::from("rm $1"),
              confirm: true,
              ..Default::default()
            },
          ],
          lines_to_skip: 0,
          // display_command: Some(DisplayCommand {
          //   command: String::from("git diff $1"),
          //   regex: None,
          // }),
          display_command: None,
        },
        Profile {
          name: String::from("git status"),
          registered_commands: vec![String::from("git status")],
          key_bindings: vec![
            KeyBinding {
              key: 'A',
              command: String::from("git add $0"),
              ..Default::default()
            },
            KeyBinding {
              key: 'a',
              command: String::from("git reset $1"),
              confirm: true,
              regex: Some(String::from(".*:\\s+([^\\s]+)")),
              ..Default::default()
            },
            KeyBinding {
              key: 'd',
              command: String::from("rm $1"),
              confirm: true,
              ..Default::default()
            },
          ],
          lines_to_skip: 0,
          // display_command: Some(DisplayCommand {
          //   command: String::from("git diff $1"),
          //   regex: None,
          // }),
          display_command: None,
        },
        Profile {
          name: String::from("docker ps"),
          registered_commands: vec![String::from("docker ps")],
          key_bindings: vec![
            KeyBinding {
              key: 's',
              command: String::from("docker stop $0"),
              confirm: true,
              ..Default::default()
            },
            KeyBinding {
              key: 'r',
              command: String::from("docker restart $0"),
              confirm: false,
              ..Default::default()
            },
            KeyBinding {
              key: 'd',
              command: String::from("docker kill $0"),
              confirm: true,
              ..Default::default()
            },
          ],
          lines_to_skip: 0,
          display_command: None,
          // display_command: Some(DisplayCommand {
          //   command: String::from("docker inspect $0"),
          //   regex: None,
          // }),
        },
        Profile {
          name: String::from("git branch"),
          registered_commands: vec![String::from("git branch")],
          key_bindings: vec![KeyBinding {
            key: 'c',
            command: String::from("git checkout $1"),
            ..Default::default()
          }],
          lines_to_skip: 0,
          // display_command: Some(DisplayCommand {
          //   command: String::from("git log --oneline $0"),
          //   regex: None,
          // }),
          display_command: None,
        },
        Profile {
          name: String::from("lsof -iTCP | grep LISTEN"),
          registered_commands: vec![
            String::from("lsof -iTCP | grep LISTEN"),
            String::from("lsof -iTCP"),
          ],
          key_bindings: vec![KeyBinding {
            key: 'd',
            command: String::from("kill -9 $1"),
            confirm: true,
            ..Default::default()
          }],
          lines_to_skip: 0,
          display_command: None,
        },
      ],
    }
  }
}
