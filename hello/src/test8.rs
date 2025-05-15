use std::fs;


use std::collections::HashMap;

fn test() {
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
  // let c = &v[123];
  // 第8章第1节
  let s1 = String::from("😀Hello Cargo");
  // let slice1 = &s1[0..1];
  // println!("{}", slice1);
  for _c in s1.chars() {
    println!("{}", _c);
  }
  let mut map = HashMap::<String, i32>::new();
  map.insert(String::from("Blue"), 123);
  map.insert(String::from("Red"), 456);
  // map.insert(String::from("Blue"), 789);
  for (key, value) in &map {
    println!("{}: {}", key, value);
  }
  map.iter().for_each(|(key, value)| {
    println!("{}: {}", key, value);
  });

  let j = "hello world wonderful world";
  let mut words = HashMap::<&str, i32>::new();

  for word in j.split_whitespace() {
    let count = words.entry(word).or_insert(0);
    *count += 1;
  }
  for (word, count) in &words {
    println!("{} : {}", &word, &count);
  }
}

fn test(o1: Option<&mut i32>) {
  let a = o1.unwrap();
  *a = *a * 2;
}