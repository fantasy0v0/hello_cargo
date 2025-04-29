
fn main() {
  let b = Box::new(5);
  let c = 123;
  let d = List::Cons(1, Box::new(List::Nil));
  if let List::Cons(x, y) = d {
    println!("{}",x);
  }

  // deref tarit
  let x = 5;
  let y = &x;

  assert_eq!(5, x);
  assert_eq!(x, *y);

  let y = Box::new(5);
  assert_eq!(x, *y);
}

enum List{
  Cons(i32, Box<List>),
  Nil,
}