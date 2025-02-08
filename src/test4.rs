mod test1;

enum Message {
  Quit,
  Move { x: i32, y: i32 },
  Write(String),
  ChangeColor(i32, i32, i32),
}

impl Message {
  fn call(&self) {
    // 在这里定义方法体
    match self {
        Message::Write(s) => println!("Write: {}", s),
        _ => println!("Other message"),
    }
  }
}

enum Coin {
  Penny,
  Nickel,
  Dime,
  Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
  match coin {
      Coin::Penny => 1,
      Coin::Nickel => 5,
      Coin::Dime => 10,
      Coin::Quarter => 25,
  }
}

fn test() {

  let m = Message::Write(String::from("test"));
  m.call();

  let coin = Coin::Penny;
  println!("Value in cents: {}", value_in_cents(coin));
  
  let config_max = Some(3u8);
  if let Some(a) = config_max {
    println!("Some: {}", a);
  } else {
    println!("None");
    return;
  }
  test1::test1();

  test1::test11::test11();
}