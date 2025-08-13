use std::{thread, time::Duration};


fn main() {
  let v= vec![1,2,3];
  let t = thread::spawn(|| {
    for i in 1..10 {
      println!("现在是:{}", i);
      thread::sleep(Duration::from_millis(1000));
    }
  });
  let _ = t.join();
  let t = thread::spawn(move || {
    println!("vec: {v:?}");
  });
  let _ = t.join();
  println!("it's done");
}
