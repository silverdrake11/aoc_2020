use std::time::Instant;

mod day12;
use day12::day12;


fn main() {

  let now = Instant::now();
  day12();
  println!("{:?}", now.elapsed());

}
