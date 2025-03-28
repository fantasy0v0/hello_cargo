use std::env;
use std::fs;
use std::process;
use std::error::Error;

fn main() {
  let args: Vec<String> = env::args().collect();
  let config = Config::build(&args).unwrap_or_else(|err| {
    println!("{}", err);
    process::exit(1);
  });
  println!("Searching for {}", config.query);
  println!("In file {}", config.file_path);

  if let Err(err) = run(config) {
    println!("Application error: {}", err);
    process::exit(1);
  }
}

struct Config {
  query: String,
  file_path: String,
}

impl Config {
  fn new(args: &[String]) -> Config {
    if env::args().len() < 3 {
      panic!("Not enough arguments.");
    }
    return parse_config(args);
  }

  fn build(args: &[String]) -> Result<Config, &str> {
    if env::args().len() < 3 {
      return Err("Not enough arguments.");
    }
    return Ok(parse_config(args));
  }
}

fn parse_config(args: &[String]) -> Config {
  Config {
    query: args[1].clone(),
    file_path: args[2].clone(),
  }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
  // let contents = fs::read_to_string(config.file_path).expect("Should have been able to read to file");
  let contents = fs::read_to_string(config.file_path)?;
  println!("file contents: \n{}", contents);
  Ok(())
}