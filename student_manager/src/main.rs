mod student;
use student::User;
mod ui;
use ui::{read_line, write_line};

fn main() {
  println!("Hello, world!");
  let user = User::new("admin", "123456");
  let username = read_line("请输入用户名：").trim().to_string();
  let password = read_line("请输入密码：").trim().to_string();
  if user.matches(&username, &password) {
    write_line("登录成功");
  } else {
    write_line("登录失败");
  }
}