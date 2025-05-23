use std::{cmp::Ordering, io};
use rand::Rng;

pub fn test1() {
  println!("Guess the number!");
  let secret_number = rand::thread_rng().gen_range(1..=100);
  println!("The secret number is: {secret_number}");

  println!("Please input your guess.");

  let mut guess = String::new();
  io:: stdin().read_line(&mut guess).expect("Failed to read line");
  println!("You guessed: {guess}");

  let guess: u32 = guess.trim().parse().expect("Please type a number!");

  match guess.cmp(&secret_number) {
      Ordering::Less => println!("Too small!"),
      Ordering::Greater => println!("Too big!"),
      Ordering::Equal => println!("You win!")
  }

  let x = 5;
  println!("x = {x}");
  let x = 6;
  println!("x = {x}");
}

pub mod test11 {
  pub fn test11() {
    println!("test11");
  }
}