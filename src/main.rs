
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
  println!("user.name == user2.name = {}", ret);
  println!("user.name = {}", user.name);
}

struct User {

  id: i32,

  name: String

}