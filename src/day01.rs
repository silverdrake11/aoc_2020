use std::fs;
use itertools::Itertools;


pub fn day01() {

  let filename: String = "1.txt".to_string();

  let contents = fs::read_to_string(filename).unwrap();

  let mut v: Vec<u32> = Vec::new();
  for value in contents.lines() {
    let x = value.parse::<u32>().unwrap();
    v.push(x);
  }

  // Part 1
  for (i, num1) in v.iter().enumerate() {
    for j in i+1..v.len() {
      let num2 = v[j];
      if num1 + num2 == 2020 {
        println!("Part 1) {}", num1 * num2);
      }
    }
  }

  // Part 2

  let combs = v.into_iter().combinations(3); // Move v into combs

  for value in combs {
    let answer: u32 = value.iter().sum();
    if answer == 2020 {
      let prod: u32 = value.iter().product();
      println!("Part 2) {}", prod);
    }
  }

}