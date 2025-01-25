fn main() {
  let mut user = User {
    name: "John".to_string(),
    id: 1
  };
  user.name = "Jane".to_string();
  // 如果不clone，user.name的所有权会被转移到user2.name，导致user.name无法被使用
  let user2 = User {
    id: 2,
    name: user.name.clone()
  };
  let ret = user.name == user2.name;
  println!("user.name == user2.name = {ret}");
  println!("user.name = {}", user.name);

  let user3 = User {
    ..user
  };
  println!("id: {}, name: {}", user3.id, user3.name);

  let red = Color(255, 0, 0);
  println!("red = {}", red.0);

  let rect = Rectangle {
    width: 10, height: 5
  };
  rect.area();
  (&rect).area();
  let sq = Rectangle::square(50);
  println!("sq = {:?}", sq);
  println!("{:x}", 254);
}

struct User {

  id: i32,

  name: String

}

struct Color(i32, i32, i32);

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
  fn area(&self) -> u32 {
      self.width * self.height
  }

  fn square(size: u32) -> Self {
    Self {
      width: size, height: size
    }
  }
}