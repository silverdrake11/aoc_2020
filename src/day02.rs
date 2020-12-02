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
    let c = v[2];
    let password = v[4];

    // Part 1
    let count = password.matches(c).count();
    if count >= min && count <= max {
      total+=1;
    }

    // Part 2
    let letter1 = &password[min-1..min]; // Slice of size 1
    let letter2 = &password[max-1..max];
    if letter1 == c || letter2 == c {
      if letter1 != letter2 {
        total2 += 1;
      }
    }

  }

  println!("Part 1) {}", total);
  println!("Part 2) {}", total2);

}