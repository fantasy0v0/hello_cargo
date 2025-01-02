
fn main() {
  let s1 = String::from("Hello");
  
  let s2 = &s1;
  let s3 = &s1;
  println!("s2 = {s2}, s3 = {s3}");
}
