use std::process::Stdio;


fn main() {
  println!("Hello, world!");
  // 执行系统的ping命令
  // let result = std::process::Command::new("ping")
  let mut result = std::process::Command::new("ping")
    .arg("127.0.0.1")
    // .stdout(Stdio::piped())
    .spawn()
    .expect("ping failed to start");  
  // let out = result.wait_with_output().expect("ping failed to wait");
  let status = result.wait().expect("ping failed to wait");  
  println!("process: {}, code: {} success:{}", result.id(), status.code().unwrap_or_default(), status.success());
  // println!("输出: {}", String::from_utf8_lossy(&out.stdout));
  println!("执行结束!");
}