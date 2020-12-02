use std::fs;


pub fn day02() {

  let filename: String = "2.txt".to_string();

  let contents = fs::read_to_string(filename).unwrap();
  let mut total: usize = 0;
  let mut total2: usize = 0;

  for value in contents.lines() {
    let v: Vec<&str> = value.split(|c| c == ':' || c == ' ' || c == '-').collect();
    let min = v[0].parse::<usize>().unwrap(); // Convert to int
    let max = v[1].parse::<usize>().unwrap(); // Convert to int
    let c = v[2].chars().nth(0).unwrap(); // Convert to char
    let password = v[4];

    let count = password.matches(c).count();
    if count >= min && count <= max {
      total+=1;
    }

    let char1 = password.chars().nth(min-1).unwrap();
    let char2 = password.chars().nth(max-1).unwrap();

    if char1 == c as char || char2 == c as char {
      if char1 != char2 {
        total2+=1;
      }
    }

  }

  println!("Part 1) {}", total);
  println!("Part 2) {}", total2);

}