// 最前面加上crate:: 表示从项目根目录开始查找, 相当于绝对路径, 如果没有的话就相当于相对路径
use crate::garden::vegetables::Asparagus;

pub mod garden;
pub mod test1;
pub mod test2;
pub mod test3;
pub mod test4;

fn test5() {
  let asparagus = Asparagus {};
  println!("{:?}", asparagus);
}