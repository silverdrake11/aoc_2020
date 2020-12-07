use std::fs;
use std::collections::HashSet;


pub fn day06() {

  let filename: String = "6.txt".to_string();

  let contents = fs::read_to_string(filename).unwrap();

  let mut set: HashSet<char> = HashSet::new();

  let mut questions: usize = 0;

  for line in contents.lines() {

    for item in line.split(" ") {

      if item == "" {

        set.clear();

      } else {

      for c in item.chars() {
        if !set.contains(&c) {
          set.insert(c);
          questions += 1;
        }
      }

      }
    }
  }
  println!("{}", questions);
}