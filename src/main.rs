use std::time::Instant;

mod day15;
use day15::day15;


fn main() {

  let now = Instant::now();
  day15();
  println!("{:?}", now.elapsed());

}
