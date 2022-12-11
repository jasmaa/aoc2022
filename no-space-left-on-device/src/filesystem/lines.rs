use std::io::{BufRead, BufReader};

#[derive(Debug)]
pub enum LineParse {
  CD { path: String },
  LS,
  Dir { name: String },
  File { name: String, size: u64 },
}

fn parse_command(line: &String) -> LineParse {
  let args = line.split(" ").collect::<Vec<&str>>();
  match args[1] {
    "cd" => {
      if args.len() != 3 {
        panic!("unable to parse cd");
      }
      LineParse::CD {
        path: String::from(args[2]),
      }
    }
    "ls" => LineParse::LS,
    _ => {
      panic!("unable to parse command")
    }
  }
}

fn parse_dir(line: &String) -> LineParse {
  let args = line.split(" ").collect::<Vec<&str>>();
  if args.len() != 2 {
    panic!("unable to parse dir");
  }
  match args[0] {
    "dir" => LineParse::Dir {
      name: String::from(args[1]),
    },
    _ => {
      panic!("encountered unknown symbol")
    }
  }
}

fn parse_file(line: &String) -> LineParse {
  let args = line.split(" ").collect::<Vec<&str>>();
  if args.len() != 2 {
    panic!("unable to parse file");
  }
  LineParse::File {
    name: String::from(args[1]),
    size: args[0].parse::<u64>().unwrap(),
  }
}

pub fn parse<R: BufRead>(r: BufReader<R>) -> Vec<LineParse> {
  let mut tokens: Vec<LineParse> = Vec::new();
  for line in r.lines() {
    let l = line.unwrap();
    let lookahead = l.as_str().chars().nth(0);
    match lookahead {
      Some(v) => match v {
        '$' => {
          tokens.push(parse_command(&l));
        }
        'd' => {
          tokens.push(parse_dir(&l));
        }
        '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' | '0' => {
          tokens.push(parse_file(&l));
        }
        _ => {
          panic!("encountered unknown symbol")
        }
      },
      None => {
        // no-op
      }
    }
  }
  tokens
}
