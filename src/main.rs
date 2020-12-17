use std::time::Instant;

mod day17;
use day17::advent;


fn main() {

  let now = Instant::now();
  advent();
  println!("{:?}", now.elapsed());

}
