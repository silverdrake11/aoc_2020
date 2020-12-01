use std::time::Instant;

mod day01;
use day01::day01;


fn main() {

  let now = Instant::now();
  day01();
  println!("{:?}", now.elapsed());

}
