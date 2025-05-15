use std::{fmt::{write, Display}};

use rand::Rng;

fn main() {
  let r = &123;
  {
    let x = 5;
    // r = &x;
  }
  println!("r: {r}");

  let string1 = String::from("abcd");
  let string3: &str;
  {
    let string2 = "xyz";
    string3 = longest(string1.as_str(), string2);
  }
  println!("The longest string is {}", string3);

  let a = 5;
  let c;
  {
    let b = 10;
    let d = 123;
    c = test(&a, &b, &d);
  }

  let dog = Dog {};
  println!("{}", dog);
  let cat = Cat {};
  println!("dog: {}, cat: {}", dog.name(), cat.name());
  println!("cat: {}", give_animal().name());
  print_animal_name(&dog);
  print_animal_name(&cat);

  print_animal_name_1(&dog);
  print_animal_name_1(&cat);

  print_animal_name_2(&dog);
  // print_animal_name_2(&cat);
  
  3.to_string_fan();
  dog.to_string_fan();

  let novel = String::from("Call me Ishmael. Some years ago...");
  let first_sentence = novel.split('.').next().unwrap();
  let i = ImportantExcerpt {
      part: first_sentence,
  };
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
  if x.len() > y.len() {
    x
  } else {
    y
  }
}

fn test<'a, T: PartialOrd>(x: &'a T, y: &'a T, z: &T) -> &'a T {
  if x > y {
    x
  } else {
    y
  }
}

struct Dog {}

struct Cat {}

trait Animal {
  fn name(&self) -> &str;
}

impl Animal for Dog {
  fn name(&self) -> &str {
    "This is a 🐶"
  }
}

impl std::fmt::Display for Dog {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    return write!(f, "I am a dog.");
  }
}

impl Animal for Cat {
  fn name(&self) -> &str {
    "This is a 🐱"
  }
}

fn give_animal() -> impl Animal {
  // let number = rand::thread_rng().gen_range(1..=10);
  // if (number > 5) {
  //   return Dog {};
  // } else {
  //   return Cat {};
  // }
  Cat {}
}

fn print_animal_name(animal: &impl Animal) {
  println!("{}", animal.name());
}

fn print_animal_name_1<T: Animal>(animal: &T) {
  println!("{}", animal.name());
}

fn print_animal_name_2<T>(animal: &T)
where 
  T: Animal + Display
{
  println!("{}", animal.name());
}

trait ToString {
  fn to_string_fan(&self) -> String;
}

impl<T: Display> ToString for T {
  fn to_string_fan(&self) -> String {
    format!("{}", self)
  }
}

struct ImportantExcerpt<'a> {
  part: &'a str,
}