use std::time::Instant;

mod day14;
use day14::day14;


fn main() {

  let now = Instant::now();
  day14();
  println!("{:?}", now.elapsed());

}
