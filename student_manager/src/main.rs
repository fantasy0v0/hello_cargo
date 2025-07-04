use std::io::{self, Write};

mod student;
use student::User;

fn main() {
  println!("Hello, world!");
  let user = User::new("admin", "123456");
  let mut username = String::new();
  io::stdout().write("请输入用户名：".as_bytes()).expect("输出失败");
  io::stdout().flush().expect("flush失败");
  io::stdin().read_line(&mut username).expect("请输入用户名");
  let mut password = String::new();
  io::stdout().write("请输入密码：".as_bytes()).expect("输出失败");
  io::stdout().flush().expect("flush失败");
  io::stdin().read_line(&mut password).expect("请输入密码");
  let username = username.trim();
  let password = password.trim();
  if user.matches(username, password) {
    println!("登录成功");
  } else {
    println!("登录失败");
  }
}
