use std::fs;


pub fn day05() {

  let filename: String = "5.txt".to_string();
  let text = fs::read_to_string(filename).unwrap();

  for line in text.lines() {
    println!("{}", line)
  }

}