use std::time::Instant;

mod day03;
use day03::day03;


fn main() {

  let now = Instant::now();
  day03();
  println!("{:?}", now.elapsed());

}
