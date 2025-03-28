use std::{env, error::Error, fs};

pub struct Config {
  pub query: String,
  pub file_path: String,
}

impl Config {
  fn new(args: &[String]) -> Config {
    if env::args().len() < 3 {
      panic!("Not enough arguments.");
    }
    return parse_config(args);
  }

  pub fn build(args: &[String]) -> Result<Config, &str> {
    if env::args().len() < 3 {
      return Err("Not enough arguments.");
    }
    return Ok(parse_config(args));
  }
}

pub fn parse_config(args: &[String]) -> Config {
  Config {
    query: args[1].clone(),
    file_path: args[2].clone(),
  }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
  // let contents = fs::read_to_string(config.file_path).expect("Should have been able to read to file");
  let contents = fs::read_to_string(config.file_path)?;
  println!("file contents: \n{}", contents);
  Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
  let mut results: Vec<&str> = Vec::new();
  for line in contents.lines() {
    if line.contains(query) {
      results.push(line);
    }
  }
  results
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn one_result() {
    let query = "duct";
    let contents = "\
Rust:
safe, fast, productive.
Pick three.";
    assert_eq!(vec!["safe, fast, productive."], search(query, contents));
  }
}