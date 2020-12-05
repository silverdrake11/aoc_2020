use std::time::Instant;

mod day05;
use day05::day05;


fn main() {

  let now = Instant::now();
  day05();
  println!("{:?}", now.elapsed());

}
