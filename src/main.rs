use std::time::Instant;

mod day04;
use day04::day04;


fn main() {

  let now = Instant::now();
  day04();
  println!("{:?}", now.elapsed());

}
