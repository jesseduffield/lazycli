mod char_pos_iter;

use char_pos_iter::CharPosIter;
use std::{collections::HashSet, iter::FromIterator};

#[derive(PartialEq, Debug)]
pub struct Row {
  pub original_line: String,
  pub cells: Vec<String>,
}

impl Row {
  pub fn new(original_line: String, cells: Vec<String>) -> Row {
    Row {
      original_line,
      cells,
    }
  }

  pub fn cells_as_strs(&self) -> Vec<&str> {
    self.cells.iter().map(|c| c.as_ref()).collect()
  }
}

pub fn parse(text: String) -> Vec<Row> {
  let column_indices = get_column_indices(&text);

  text
    .lines()
    .map(|line| {
      // I want to get the chars as an array, then slice that up.
      let chars = line.chars().collect::<Vec<char>>();

      let char_len = [chars.len()];

      let positions = column_indices
        .iter()
        .chain(char_len.iter())
        .collect::<Vec<_>>();

      let cells = positions
        .iter()
        .take(positions.len() - 1)
        .enumerate()
        .map(|(i, _)| {
          safe_vec_range(&chars, positions[i], positions[i + 1])
            .into_iter()
            .collect::<String>()
            .trim_end()
            .to_owned()
        })
        .collect();

      Row::new(line.to_owned(), cells)
    })
    .collect()
}

fn safe_vec_range<'a, T>(v: &'a Vec<T>, from: &usize, to: &usize) -> Vec<&'a T> {
  if from > to {
    return vec![];
  }

  v.iter().skip(*from).take(to - from).collect::<Vec<_>>()
}

fn get_column_indices(text: &String) -> Vec<usize> {
  let mut lines = text.trim_end().lines();

  let first_line = lines.next().unwrap_or_default();

  let spaces_iter = CharPosIter::new(first_line)
    // ignoring index 0 for the sake of something like git status --short with a single line i.e. ` M myfile.txt`.
    .filter(|&(index, char)| index != 0 && char == ' ')
    .map(|(index, _char)| index);

  let mut spaces_set: HashSet<usize> = HashSet::from_iter(spaces_iter);

  for line in lines {
    // TODO consider how to remove the .clone() here
    for s_index in spaces_set.clone() {
      for (index, char) in CharPosIter::new(line) {
        if index == s_index && char != ' ' {
          spaces_set.remove(&s_index);
        }
      }
    }
  }

  let mut spaces = spaces_set.into_iter().collect::<Vec<usize>>();
  spaces.sort();

  let mut result = spaces
    .iter()
    .enumerate()
    .filter(|(index, position)| {
      *index == spaces.len() - 1 || spaces[*index + 1] != (**position + 1)
    })
    .map(|(_index, position)| *position + 1)
    .collect::<Vec<usize>>();

  result.insert(0, 0);

  result
}

#[cfg(test)]
mod tests {
  use super::*;
  use pretty_assertions::assert_eq;

  #[test]
  fn test_one_line_cut_short() {
    let text = "col1 col2 col3\n\
                col1 col2 col3\n\
                col1\n";

    assert_eq!(
      parse(String::from(text)),
      vec![
        Row {
          original_line: String::from("col1 col2 col3"),
          cells: vec![
            String::from("col1"),
            String::from("col2"),
            String::from("col3"),
          ],
        },
        Row {
          original_line: String::from("col1 col2 col3"),
          cells: vec![
            String::from("col1"),
            String::from("col2"),
            String::from("col3"),
          ],
        },
        Row {
          original_line: String::from("col1"),
          cells: vec![String::from("col1"), String::from(""), String::from(""),],
        },
      ],
    )
  }

  #[test]
  fn test_parse_docker_ps() {
    let text = "CONTAINER ID   IMAGE                       COMMAND              CREATED       STATUS          PORTS                              NAMES\n\
                17c523089229   aa                          \"./ops/dev/api\"      2 weeks ago   Up 43 seconds   0.0.0.0:20->80/tcp                 blah\n\
                dcddf219bb2b   bb                          \"./ops/dev/sid…\"     2 weeks ago   Up 44 seconds                                      blah-sidekiq_2\n\
                43484e7c2774   dd:latest                   \"ops/dev/proxy…\"     2 weeks ago   Up 46 seconds   0.0.0.0:80->80/tcp, 9400/tcp       blah-proxy_4\n\
                8a61b6cc2d3b   aaaaa:4.0.3-alpine          \"docker.s…\"          2 weeks ago   Up 46 seconds   0.0.0.0:6300->6322/tcp             blah.99_1\n";

    assert_eq!(parse(String::from(text)), vec![
          Row {
                original_line: String::from("CONTAINER ID   IMAGE                       COMMAND              CREATED       STATUS          PORTS                              NAMES"),
                cells: vec![
                    String::from("CONTAINER ID"),
                    String::from("IMAGE"),
                    String::from("COMMAND"),
                    String::from("CREATED"),
                    String::from(""),
                    String::from("STATUS"),
                    String::from("PORTS"),
                    String::from("NAMES"),
                ],
            },
            Row {
                original_line: String::from("17c523089229   aa                          \"./ops/dev/api\"      2 weeks ago   Up 43 seconds   0.0.0.0:20->80/tcp                 blah"),
                cells: vec![
                    String::from("17c523089229"),
                    String::from("aa"),
                    String::from("\"./ops/dev/api\""),
                    String::from("2 weeks"),
                    String::from("ago"),
                    String::from("Up 43 seconds"),
                    String::from("0.0.0.0:20->80/tcp"),
                    String::from("blah"),
                ],
            },
            Row {
                original_line: String::from("dcddf219bb2b   bb                          \"./ops/dev/sid…\"     2 weeks ago   Up 44 seconds                                      blah-sidekiq_2"),
                cells: vec![
                    String::from("dcddf219bb2b"),
                    String::from("bb"),
                    String::from("\"./ops/dev/sid…\""),
                    String::from("2 weeks"),
                    String::from("ago"),
                    String::from("Up 44 seconds"),
                    String::from(""),
                    String::from("blah-sidekiq_2"),
                ],
            },
            Row {
                original_line: String::from("43484e7c2774   dd:latest                   \"ops/dev/proxy…\"     2 weeks ago   Up 46 seconds   0.0.0.0:80->80/tcp, 9400/tcp       blah-proxy_4"),
                cells: vec![
                    String::from("43484e7c2774"),
                    String::from("dd:latest"),
                    String::from("\"ops/dev/proxy…\""),
                    String::from("2 weeks"),
                    String::from("ago"),
                    String::from("Up 46 seconds"),
                    String::from("0.0.0.0:80->80/tcp, 9400/tcp"),
                    String::from("blah-proxy_4"),
                ],
            },

            Row {
                original_line: String::from("8a61b6cc2d3b   aaaaa:4.0.3-alpine          \"docker.s…\"          2 weeks ago   Up 46 seconds   0.0.0.0:6300->6322/tcp             blah.99_1"),
                cells: vec![
                    String::from("8a61b6cc2d3b"),
                    String::from("aaaaa:4.0.3-alpine"),
                    String::from("\"docker.s…\""),
                    String::from("2 weeks"),
                    String::from("ago"),
                    String::from("Up 46 seconds"),
                    String::from("0.0.0.0:6300->6322/tcp"),
                    String::from("blah.99_1"),
                ],
            },


    ])
  }

  #[test]
  fn test_parse_git_status() {
    let text = " M src/main.rs\n\
                ?? src/parse/\n";

    assert_eq!(
      parse(String::from(text)),
      vec![
        Row {
          original_line: String::from(" M src/main.rs"),
          cells: vec![String::from(" M"), String::from("src/main.rs"),],
        },
        Row {
          original_line: String::from("?? src/parse/"),
          cells: vec![String::from("??"), String::from("src/parse/"),],
        },
      ],
    )
  }

  #[test]
  fn test_parse_git_status_one_line() {
    let text = " M src/main.rs\n";

    assert_eq!(
      parse(String::from(text)),
      vec![Row {
        original_line: String::from(" M src/main.rs"),
        cells: vec![String::from(" M"), String::from("src/main.rs"),],
      },],
    )
  }

  #[test]
  fn test_parse_ls() {
    let text = "-rw-r--r--     1 user  staff     159 28 Apr  2020 Dockerfile\n\
                -rw-r--r--     1 user  staff    7910 21 Sep 15:19 Readme.md\n\
                drwxr-xr-x     3 user  staff      96 11 Apr  2020 docs";

    assert_eq!(
      parse(String::from(text)),
      vec![
        Row {
          original_line: String::from(
            "-rw-r--r--     1 user  staff     159 28 Apr  2020 Dockerfile"
          ),
          cells: vec![
            String::from("-rw-r--r--"),
            String::from("1"),
            String::from("user"),
            String::from("staff"),
            String::from(" 159"),
            String::from("28"),
            String::from("Apr"),
            String::from(" 2020"),
            String::from("Dockerfile"),
          ],
        },
        Row {
          original_line: String::from(
            "-rw-r--r--     1 user  staff    7910 21 Sep 15:19 Readme.md"
          ),
          cells: vec![
            String::from("-rw-r--r--"),
            String::from("1"),
            String::from("user"),
            String::from("staff"),
            String::from("7910"),
            String::from("21"),
            String::from("Sep"),
            String::from("15:19"),
            String::from("Readme.md"),
          ],
        },
        Row {
          original_line: String::from("drwxr-xr-x     3 user  staff      96 11 Apr  2020 docs"),
          cells: vec![
            String::from("drwxr-xr-x"),
            String::from("3"),
            String::from("user"),
            String::from("staff"),
            String::from("  96"),
            String::from("11"),
            String::from("Apr"),
            String::from(" 2020"),
            String::from("docs"),
          ],
        },
      ],
    )
  }
}
