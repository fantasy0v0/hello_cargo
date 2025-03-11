use std::{fs::{self, File}, io::{self, read_to_string, Error, Read}, cmp::Ordering};
use rand::Rng;


fn main() {
  // panic!("I want to panic!");
  let greeting_file = File::open("hello.txt");
  let a = match greeting_file {
    Ok(file) => file,
    Err(error) => {
      println!("Problem opening the file: {:?}", error);
      File::create("hello.txt").unwrap()
    }
  };
  println!("File opened!");
  read_username_from_file();
  fs::remove_file("hello.txt");

  loop {
    let mut guess = String::new();
    let secret_number = rand::thread_rng().gen_range(1..=10);
    io::stdin().read_line(&mut guess).expect("Failed to read line");
    println!("guess: {}", guess);
    let guess: i32 = match guess.trim().parse() {
      Ok(num) => num,
      Err(err) => {
        println!("{:?}", err);
        continue;
      }
    };
    if guess < 1 || guess > 10 {
      println!("数值不对");
      continue;
    }
    match guess.cmp(&secret_number) {
      Ordering::Less => println!("猜小了"),
      Ordering::Equal => println!("恭喜你猜中了"),
      Ordering::Greater => println!("猜大了")
    };
  }
}

fn read_username_from_file() -> Result<String, std::io::Error> {
  let username_file_result = File::open("hello.txt");
  let mut username_file = match username_file_result {
    Ok(file) => file,
    Err(error) => return Err(error),
  };
  let mut username_string = String::new();
  let a = match username_file.read_to_string(&mut username_string) {
    Ok(_) => Ok(username_string.clone()),
    Err(error) => Err(error),
  };
  let a = match read_to_string(&username_file) {
    Ok(_) => Ok(username_string.clone()),
    Err(error) => Err(error)
  };
  return a;
}