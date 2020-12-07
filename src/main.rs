use std::time::Instant;

mod day06;
use day06::day06;


fn main() {

  let now = Instant::now();
  day06();
  println!("{:?}", now.elapsed());

}
