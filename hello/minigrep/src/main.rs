use minigrep::{run, Config};
use std::env;
use std::process;

fn main() {
  let args: Vec<String> = env::args().collect();
  let config = Config::build(&args).unwrap_or_else(|err| {
    eprintln!("{}", err);
    process::exit(1);
  });
  println!("Searching for {} in file {}", config.query, config.file_path);

  if let Err(err) = run(config) {
    eprintln!("Application error: {}", err);
    process::exit(1);
  }
}
