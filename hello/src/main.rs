use std::{process::Stdio, rc::Rc};


fn main() {
  let a = Rc::new(123);
  let b = Rc::clone(&a);
  println!("b: {}, rc: {}", b, Rc::strong_count(&a));
}