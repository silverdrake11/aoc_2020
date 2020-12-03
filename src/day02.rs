use std::fs;


pub fn day02() {

  let filename: String = "2.txt".to_string();

  let contents = fs::read_to_string(filename).unwrap();
  let mut total1: usize = 0;
  let mut total2: usize = 0;

  for value in contents.lines() {

    let v: Vec<&str> = value.split(|c| c == ':' || c == ' ' || c == '-').collect();
    let min = v[0].parse::<usize>().unwrap(); // Convert to int
    let max = v[1].parse::<usize>().unwrap(); // Convert to int
    let c = v[2];
    let password = v[4];

    // Part 1
    let count = password.matches(c).count();
    if count >= min && count <= max {
      total1 += 1;
    }

    // Part 2
    let cbytes = c.as_bytes()[0];
    let pass_bytes = password.as_bytes();

    let letter1 = pass_bytes[min-1];
    let letter2 = pass_bytes[max-1];
    if letter1 == cbytes || letter2 == cbytes {
      if letter1 != letter2 {
        total2 += 1;
      }
    }

  }

  println!("Part 1) {}", total1);
  println!("Part 2) {}", total2);

}