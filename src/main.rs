use std::{fmt::write, path::Display};

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