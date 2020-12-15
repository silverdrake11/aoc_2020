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

fn check_data(key: &str, value: &str, hcl_re: &Regex, pid_re: &Regex) -> bool {

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
    "hcl" => hcl_re.is_match(value),
    "pid" => pid_re.is_match(value),
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
    _ => false
  }
}

pub fn advent() {

  let filename: String = "4.txt".to_string();

  let contents = fs::read_to_string(filename).unwrap();

  let mut table: HashSet<String> = HashSet::new();
  let fields = ["byr","iyr","eyr","hgt","hcl","ecl","pid"];
  for field in &fields {
    table.insert(field.to_string());
  }
  let hcl_re = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
  let pid_re = Regex::new(r"^\d{9}$").unwrap();

  let mut num_valid: usize = 0;
  let mut num_fields: usize = 0;

  for line in contents.lines() {

    for item in line.split(" ") {

      if item == "" {

        if num_fields == table.len() {
          num_valid += 1;
        }
        num_fields = 0;

      } else {

        let x: Vec<&str> = item.split(":").collect();
        let key = x[0];
        let value = x[1];

        if key == "cid" {
          continue;
        }
        
        //if table.contains(key) { // Part 1
        if check_data(key, value, &hcl_re, &pid_re) { // Part 2
          num_fields += 1;
        } 
      }
    }
  }
  println!("{}", num_valid);
}