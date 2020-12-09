use std::time::Instant;

mod day09;
use day09::day09;


fn main() {

  let now = Instant::now();
  day09();
  println!("{:?}", now.elapsed());

}
