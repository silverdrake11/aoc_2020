use std::time::Instant;

mod day16;
use day16::advent;


fn main() {

  let now = Instant::now();
  advent();
  println!("{:?}", now.elapsed());

}
