use std::{process::Stdio, rc::Rc};

fn main() {
  let a = Rc::new(123);
  println!("a: {}, rc: {}", a, Rc::strong_count(&a));
  let b = Rc::clone(&a);
  println!("a: {}, rc: {}", a, Rc::strong_count(&a));
  println!("b: {}, rc: {}", b, Rc::strong_count(&b));
  test(Rc::clone(&b));
  println!("a: {}, rc: {}", a, Rc::strong_count(&a));
  println!("b: {}, rc: {}", b, Rc::strong_count(&b));

  let mut d: Box<i32> = Box::new(246);
  println!("d: {}", *d);
  *d = 123;
  println!("d: {}", *d);

  // RefCell<T> 可以在不更改类型(比如类型是不可变的)的情况下, 更改或使用mut方法
}

fn test(c: Rc<i32>) {
  println!("c: {}, rc: {}", c, Rc::strong_count(&c));
}

pub trait Messenger {
  fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: Messenger> {
  messenger: &'a T,
  value: usize,
  max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where
  T: Messenger,
{
  pub fn new(messenger: &'a T, max: usize) -> LimitTracker<'a, T> {
    LimitTracker {
      messenger,
      value: 0,
      max,
    }
  }

  pub fn set_value(&mut self, value: usize) {
    self.value = value;
    let percentage_of_max = self.value as f64 / self.max as f64;
    if percentage_of_max >= 1.0 {
      self.messenger.send("Error: You are over your quota!");
    } else if percentage_of_max >= 0.9 {
      self
        .messenger
        .send("Urgent warning: You've used up over 90% of your quota!");
    } else if percentage_of_max >= 0.75 {
      self
        .messenger
        .send("Warning: You've used up over 75% of your quota!");
    }
  }
}
