use std::time::Instant;

mod day08;
use day08::day08;


fn main() {

  let now = Instant::now();
  day08();
  println!("{:?}", now.elapsed());

}
