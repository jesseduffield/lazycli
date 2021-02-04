use clap::{App as ClapApp, Arg};

pub struct Args {
  pub command: String,
  pub lines_to_skip: usize,
  pub refresh_frequency: f64,
}

impl Args {
  pub fn new() -> Args {
    let matches = ClapApp::new("lazycli")
      .version("0.1")
      .author("Jesse Duffield <jessedduffield@gmail.com>")
      .about("Lets you run custom commands on a list/table returned by another program")
      .arg(
        Arg::new("ignore")
          .short('i')
          .long("ignore")
          .value_name("IGNORE")
          .about("ignores the first `n` lines of output")
          .takes_value(true),
      )
      .arg(
        Arg::new("refresh")
          .short('r')
          .long("refresh")
          .value_name("REFRESH")
          .about("frequency of refreshing the content in seconds (values like 0.1 are permitted. Values like 0.001? Also permitted, but you need to seriously look yourself in the eyes and ask whether that's something you want. Be careful, if you stare into your own eyes long enough in the mirror, a moment eventually comes when you realise that you truly exist and are aware that you exist. A revelation you might not want to inflict on yourself, especially if it's just for the sake of knowing deep down whether you want to push the limits of a command line argument)")
          .takes_value(true),
      )
      .arg(Arg::new("command").multiple(true))
      .get_matches();

    let command = match matches.values_of("command") {
      Some(matches) => matches.collect::<Vec<&str>>().join(" "),
      None => {
        eprintln!("Usage: Command must be supplied, e.g.: `lazycli -- ls -l`");
        std::process::exit(1);
      }
    };

    let lines_to_skip = match matches.value_of("ignore") {
      None => 0,
      Some(s) => match s.parse::<usize>() {
        Ok(n) => n,
        Err(_) => {
          eprintln!("ignore argument must be a number");
          std::process::exit(1);
        }
      },
    };

    let refresh_frequency = match matches.value_of("refresh") {
      None => 0.0,
      Some(s) => match s.parse::<f64>() {
        Ok(n) => n,
        Err(_) => {
          eprintln!("refresh argument must be a number");
          std::process::exit(1);
        }
      },
    };

    Args {
      command,
      lines_to_skip,
      refresh_frequency,
    }
  }
}
