
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