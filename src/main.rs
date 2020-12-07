use std::time::Instant;

mod day07;
use day07::day07;


fn main() {

  let now = Instant::now();
  day07();
  println!("{:?}", now.elapsed());

}
