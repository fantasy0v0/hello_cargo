

fn main() {
  let mut vec: Vec<i32> = Vec::new();
  vec.push(321);
  println!("{:?}", vec);

  let mut v = vec![1, 2, 3, 4, 5, 6, 7];
  let third = &v[2];
  // *third = 12;
  // println!("{}", third);
  // println!("{:?}", v);
  let third = v.get_mut(2);
  if let None = third {
    println!("没有值");
    return;
  }
  let a = third.unwrap();
  *a = 123;
  // let third = third.unwrap();
  // *third = 123;
  // println!("{:?}", v);
  let mut o0 = Some(123);
  // let o1 = o0.as_mut();
  // as_mut只是为了修改Option中的值
  test(o0.as_mut());
  println!("{:?}", o0);

  let b = v.get(2);
  match b {
    Some(bv) => println!("有值，是: {}", bv),
    None => println!("没有值")
  }
  let c = &v[123];
  // 第8章第1节
}

fn test(o1: Option<&mut i32>) {
  let a = o1.unwrap();
  *a = 123;
}