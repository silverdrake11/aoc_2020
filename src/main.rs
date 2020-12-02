use std::time::Instant;

mod day02;
use day02::day02;


fn main() {

  let now = Instant::now();
  day02();
  println!("{:?}", now.elapsed());

}
