fn main() {
  let add = |x: i32, y: i32| { x + y };
  println!("{}", add(1, 2));
  let add = |x, y| { x + y };
  println!("{}", add(1.1, 2.1));

  let example_closure = |x| x;

  let s = example_closure(String::from("hello"));
  // let n = example_closure(5);
  println!("{}", s);

  let list = vec![1, 2, 3];
  println!("Before defining closure: {list:?}");

  let only_borrows = || println!("From closure: {list:?}");

  println!("Before calling closure: {list:?}");
  only_borrows();
  // test(list);
  println!("After calling closure: {list:?}");
  // 闭包会更还回借用的所有权
}

fn test(vec: Vec<i32>) {
  println!("Before calling closure: {vec:?}");
}