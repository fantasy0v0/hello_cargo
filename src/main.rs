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
  let mut aa = 111;
  test_FnMut(|a| {
    aa += a;
    aa
  });
  println!("aa: {}", aa);
  // Fn 可重复调用, 但不能修改外部的值
  // FnMut 可重复调用, 可修改外部的值
  // FnOnce 不可重复调用, 可修改外部的值
}

fn test_FnMut<T>(mut a: T) -> i32
where T: FnMut(i32) -> i32 {
  let b = 123;
  let c = a(b);
  println!("b: {}, c: {}", b, c);
  c
}

fn test(vec: Vec<i32>) {
  println!("Before calling closure: {vec:?}");
}