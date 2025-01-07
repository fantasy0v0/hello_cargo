
fn main() {
  let s1 = String::from("Hello World");
  let s2 = &s1;
  let s3 = &s1;
  println!("s2 = {s2}, s3 = {s3}");
  
  calculate_length(s3);

  let mut s4 = String::from("Hello World");
  let s5 = &s4;
  println!("s5 = {s5}");
  let slice1 = first_word(&mut s4);
  println!("slice1 = {slice1}");
  
  s4.clear();

  let rect = Rectangle {
    width: 10,
    height: 20
  };
  println!("rect is {:#?}", &rect);
  dbg!(&rect);
}

fn calculate_length(s: &String) -> usize {
  let iter = s.as_bytes().iter();
  for (index, &value) in iter.enumerate() {
    println!("index = {}, value = {}", index, value)
  }
  1
}

fn first_word(s: &mut String) -> &str {
  let bytes = s.as_bytes();
  for (i, &item) in bytes.iter().enumerate() {
    if item == b' ' {
        return &s[0..i];
    }
  }
  &s[..]
}

#[derive(Debug)]
struct Rectangle {
  width: u32,
  height: u32
}