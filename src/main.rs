use std::time::Instant;

mod day11;
use day11::day11;


fn main() {

  let now = Instant::now();
  day11();
  println!("{:?}", now.elapsed());

}
