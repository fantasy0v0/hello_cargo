use std::fmt::Display;

fn largest<T: PartialOrd>(list: &[T]) -> &T {
  let mut largest = &list[0];

  for item in list {
      if item > largest {
          largest = item;
      }
  }

  largest
}

struct Point<T> {
  x: T,
  y: T,
}

impl<T> Point<T> {
  fn x(&self) -> &T {
      return &self.x;
  }
}

impl Point<char> {
  fn cx(&self) -> &char {
      return &self.x;
  }
}

fn test() {
  let number_list = vec![34, 50, 25, 100, 65];

  let result = largest(&number_list);
  println!("The largest number is {result}");

  let char_list = vec!['y', 'm', 'a', 'q'];

  let result = largest(&char_list);
  println!("The largest char is {result}");

  let point_list = vec![Point { x: 5, y: 10 }, Point { x: 3, y: 4}];
  // largest(&point_list);
  let x = point_list[0].x();
  println!("Point: x = {x}");

  let char_point = Point { x: 'x', y: 'y' };
  let x= char_point.cx();
  println!("CharPoint: x = {x}");

  let article = NewsArticle {
    content: String::from("Hello World"),
    author: String::from("John Doe")
  };
  println!("Article: {}", article.summarize());

  let tweet = Tweet {
    username: String::from("Fantasy001"),
    content: String::from("Hello World"),
    author: String::from("Fantasy")
  };
  println!("Tweet: {}", tweet.summarize());
}

struct NewsArticle {

  content: String,

  author: String

}

struct Tweet {
  username: String,
  content: String,
  author: String
}

trait Summary {
  fn summarize_author(&self) -> String;
  fn summarize(&self) -> String {
      format!("(Read more from {}...)", self.summarize_author())
  }
}

impl Summary for NewsArticle {
  
  fn summarize_author(&self) -> String {
    format!("{}", self.author)
  }
}

impl Summary for Tweet {

  fn summarize_author(&self) -> String {
    format!("@{}", self.username)
  }

}