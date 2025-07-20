use std::io::{self, Write};

pub fn read_line(prompt: &str) -> String {
  io::stdout().write(prompt.as_bytes()).expect("输出失败");
  io::stdout().flush().expect("flush失败");
  let mut input = String::new();
  io::stdin().read_line(&mut input).expect("输入失败");
  input
}

pub fn read_password(prompt: &str) -> String {
  io::stdout().write(prompt.as_bytes()).expect("输出失败");
  io::stdout().flush().expect("flush失败");
  return rpassword::read_password().expect("密码输入失败");
}

pub fn write_line(data: &str) {
  io::stdout().write(data.as_bytes()).expect("输出失败");
  io::stdout().flush().expect("flush失败");
}