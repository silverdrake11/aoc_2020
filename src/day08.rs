use std::fs;


pub fn day08() {

  let filename: String = "8.txt".to_string();

  let contents = fs::read_to_string(filename).unwrap();

  for line in contents.lines() {
    println!("{:?}", line);
  }


}