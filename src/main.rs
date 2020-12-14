use std::time::Instant;

mod day13;
use day13::day13;


fn main() {

  let now = Instant::now();
  day13();
  println!("{:?}", now.elapsed());

}
