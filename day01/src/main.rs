use std::env;
use std::fs;

fn main() {
  let args: Vec<String> = env::args().collect();

  let config = Config::new(&args);

  let content = fs::read_to_string(config.file_path)
    .expect("shoudl have been a file");

  println!("{content}");
}

struct Config {
  query: String, 
  file_path: String,
}

impl Config {
  fn new(args: &[String]) -> Config {
    if args.len() < 3 {
      panic!("not enough arguments");
    }

    let query = args[1].clone();
    let file_path = args[2].clone();

    Config { query, file_path }
  }
}

fn parse_config(args: &[String]) -> Config {
  let query = args[1].clone();
  let file_path = args[2].clone();

  Config { query, file_path }
}
