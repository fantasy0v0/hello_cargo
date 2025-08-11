use std::cell::RefCell;

fn main() {
  println!("hello world");
  let a = RefCell::new(5);
  *(a.borrow_mut()) = 10;
  println!("a: {}", a.borrow());
}

use crate::List::{Cons, Nil};
use std::rc::Rc;

#[derive(Debug)]
enum List {
  Cons(i32, RefCell<Rc<List>>),
  Nil,
}

impl List {
  fn tail(&self) -> Option<&RefCell<Rc<List>>> {
    match self {
      Cons(_, item) => Some(item),
      Nil => None,
    }
  }
}
