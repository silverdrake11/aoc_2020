use std::fs;
use std::collections::HashSet;

use regex::Regex;


fn check_range(value: &str, min: i16, max: i16) -> bool {
  match value.parse::<i16>() {
  Ok(num) => {
    if num < min {
      return false
    } else if num > max {
      return false
    } else {
      return true;
    }
  },
  Err(_) => return false
  }
}

fn check_data(key: &str, value: &str) -> bool {
  match key {
    "byr" => check_range(value, 1920, 2002),
    "iyr" => check_range(value, 2010, 2020),
    "eyr" => check_range(value, 2020, 2030),
    "hgt" => {
      if value.ends_with("cm") {
        check_range(&value[..value.len() - 2], 150, 193)
      } else if value.ends_with("in") {
        check_range(&value[..value.len() - 2], 59, 76)
      } else { false } 
    },
    "hcl" => Regex::new(r"^#[0-9a-f]{6}$").unwrap().is_match(value),
    "pid" => Regex::new(r"^\d{9}$").unwrap().is_match(value),
    "ecl" => {
      match value {
        "amb" => true,
        "blu" => true,
        "brn" => true,
        "gry" => true,
        "grn" => true,
        "hzl" => true,
        "oth" => true,
        _ => false }
    }
    "cid" => true,
    _ => false
  }
}

fn fill_set(table: &mut HashSet<String>) {
  let fields = ["byr","iyr","eyr","hgt","hcl","ecl","pid"];
  for field in &fields {
    table.insert(field.to_string());
  }
}

pub fn day04() {

  let filename: String = "4.txt".to_string();

  let contents = fs::read_to_string(filename).unwrap();

  let mut table: HashSet<String> = HashSet::new();
  fill_set(&mut table);

  let mut num_valid: usize = 0;

  for line in contents.lines() {

    for item in line.split(" ") {

      if item == "" {
        if table.len() == 0 {
          num_valid += 1;
        }
        fill_set(&mut table);

      } else {

        let x: Vec<&str> = item.split(":").collect();
        let key = x[0];
        let value = x[1];

        //table.remove(key); // Uncomment for Part 1
        if check_data(key, value) {
          table.remove(key);
        } 
      }
    }

  }
      println!("{}", num_valid);
}