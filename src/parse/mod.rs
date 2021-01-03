use std::collections::HashSet;
use std::iter::FromIterator;

fn parse(text: String) -> Vec<usize> {
  // split into lines, see where the columns begin.

  // we need a map of all the places where there are spaces. for each line we will remove items from this map.

  let mut lines = text.trim_end().lines();

  let first_line = lines.next().unwrap_or_default();
  println!("{}", first_line);

  let spaces_iter = first_line
    .char_indices()
    .filter(|(_index, char)| *char == ' ')
    .map(|(index, _char)| index);

  let mut spaces_set: HashSet<usize> = HashSet::from_iter(spaces_iter);

  println!("{:?}", spaces_set);

  for line in lines {
    // TODO consider how to remove the .clone() here
    for s_index in spaces_set.clone() {
      for (index, char) in line.char_indices() {
        if index == s_index && char != ' ' {
          spaces_set.remove(&s_index);
        }
      }
    }

    println!("{}", line)
  }

  println!("{:?}", spaces_set);

  let mut spaces = spaces_set.into_iter().collect::<Vec<usize>>();
  spaces.sort();
  println!("{:?}", spaces);
  let mut result = spaces
    .iter()
    .enumerate()
    .filter(|(index, position)| {
      *index == spaces.len() - 1 || spaces[*index + 1] != (**position + 1)
    })
    .map(|(_index, position)| *position + 1)
    .collect::<Vec<usize>>();

  result.insert(0, 0);

  println!("{:?}", result);

  result
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_docker_ps() {
    let text = "CONTAINER ID   IMAGE                       COMMAND              CREATED       STATUS          PORTS                              NAMES\n\
                17c523089229   aa                          \"./ops/dev/api…\"   2 weeks ago   Up 43 seconds   0.0.0.0:20->80/tcp                 blah\n\
                dcddf219bb2b   bb                          \"./ops/dev/sid…\"   2 weeks ago   Up 44 seconds                                      blah-sidekiq_2\n\
                43484e7c2774   dd:latest                   \"ops/dev/proxy…\"   2 weeks ago   Up 46 seconds   0.0.0.0:80->80/tcp                 blah-proxy_4\n\
                f13385e55986   cc:1.4.6-alpine             \"/docker.…\"        2 weeks ago   Up 46 seconds   0.0.0.0:9300->9300/tcp, 9300/tcp   blah.2_1\n\
                bc75169bb13e   mysql:1.6                   \"docker.s…\"        2 weeks ago   Up 46 seconds   0.0.0.0:12306->3406/tcp            blah.6_1\n\
                1cec55fcafbc   a-aaa-a:latest              \"docker.s…\"        2 weeks ago   Up 46 seconds   0.0.0.0:41005->9112/tcp            blah\n\
                8a61b6cc2d3b   aaaaa:4.0.3-alpine          \"docker.s…\"        2 weeks ago   Up 46 seconds   0.0.0.0:6300->6322/tcp             blah.99_1\n";

    assert_eq!(
      parse(String::from(text)),
      vec![0, 15, 43, 60, 64, 72, 78, 94, 118, 129]
    )
  }

  #[test]
  fn test_git_status() {
    let text = " M src/main.rs\n\
                ?? src/parse/\n";

    assert_eq!(parse(String::from(text)), vec![0, 3])
  }

  #[test]
  fn test_ls() {
    let text = "-rw-r--r--     1 user  staff     159 28 Apr  2020 Dockerfile\n\
                -rw-r--r--     1 user  staff    7910 21 Sep 15:19 Readme.md\n\
                drwxr-xr-x     3 user  staff      96 11 Apr  2020 docs";

    assert_eq!(
      parse(String::from(text)),
      vec![0, 15, 17, 23, 32, 37, 40, 44, 50]
    )
  }
}
