use std::time::Instant;

mod day15;
use day15::advent;


fn main() {

  let now = Instant::now();
  advent();
  println!("{:?}", now.elapsed());

}
