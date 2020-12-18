use std::time::Instant;

mod day18;
use day18::advent;


fn main() {

  let now = Instant::now();
  advent();
  println!("{:?}", now.elapsed());

}
