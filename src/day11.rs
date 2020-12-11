use std::fs;

pub fn day11() {

  let filename: String = "11.txt".to_string();

  let contents = fs::read_to_string(filename).unwrap();

  let mut adapters: Vec<usize> = Vec::new();
  adapters.push(0);

  for line in contents.lines() {
    let x = line.parse::<usize>().unwrap();
    adapters.push(x);
  }

}