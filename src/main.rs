use std::time::Instant;

mod day10;
use day10::day10;


fn main() {

  let now = Instant::now();
  day10();
  println!("{:?}", now.elapsed());

}
