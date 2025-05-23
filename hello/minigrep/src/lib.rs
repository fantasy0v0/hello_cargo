use std::{env, error::Error, fs};

pub struct Config {
  pub query: String,
  pub file_path: String,
  pub ignore_case: bool
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
  let ignore_case = env::var("IGNORE_CASE").is_ok();
  Config {
    query: args[1].clone(),
    file_path: args[2].clone(),
    ignore_case
  }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
  // let contents = fs::read_to_string(config.file_path).expect("Should have been able to read to file");
  let contents = fs::read_to_string(config.file_path)?;
  // println!("file contents: \n{}", contents);
  let results = if config.ignore_case {
    search_case_insensitive(&config.query, &contents)
  } else {
    search(&config.query, &contents)
  };
  for line in results {
    println!("{}", line);
  }
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

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
  let mut results: Vec<&str> = Vec::new();
  for line in contents.lines() {
    if line.to_lowercase().contains(&query.to_lowercase()) {
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

  #[test]
  fn case_sensitive() {
    let query = "duct";
    let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

    assert_eq!(vec!["safe, fast, productive."], search(query, contents));
  }

  #[test]
  fn case_insensitive() {
    let query = "rUsT";
    let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

    assert_eq!(
      vec!["Rust:", "Trust me."],
      search_case_insensitive(query, contents)
    );
  }
}