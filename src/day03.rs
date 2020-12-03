use std::fs;


pub fn day03() {

  let filename: String = "3.txt".to_string();

  let contents = fs::read_to_string(filename).unwrap();

  for value in contents.lines() {

    println!("{}", value);

  }

  //println!("Part 1) {}", total1);
  //println!("Part 2) {}", total2);

}