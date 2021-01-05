use clap::App as ClapApp;
use clap::Arg;

pub struct Args {
  pub command: String,
  pub lines_to_skip: usize,
}

impl Args {
  pub fn new() -> Args {
    let matches = ClapApp::new("LazyList")
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
      .arg(Arg::new("command").multiple(true))
      .get_matches();

    let command = matches
      .values_of("command")
      .unwrap()
      .collect::<Vec<&str>>()
      .join(" ");

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

    Args {
      command,
      lines_to_skip,
    }
  }
}
