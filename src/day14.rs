use std::fs;
use std::collections::HashMap;

fn check_bit(num: usize, bit: usize) -> usize {
  let one: usize = 1;
  return (num >> bit) & one;
}

fn write_bit(num: usize, bit: usize, value: usize) -> Result<usize,String> {
  let one: usize = 1;
  match value {
    1 => Ok(num | one << bit),
    0 => Ok(num & !(one << bit)),
    _ => Err("Invalid value!".to_string()),
  }
}

fn part1_write(mem_key: usize, mem_value: usize,
    mask: &Vec<char>, mem_map: &mut HashMap<usize,usize>) {

  let mut new_value: usize = mem_value;
  for (n, bit) in mask.iter().rev().enumerate() {
    match bit {
      '1' => new_value = write_bit(new_value,n,1).unwrap(),
      '0' => new_value = write_bit(new_value,n,0).unwrap(),
        _ => (),
    }
  }
  mem_map.insert(mem_key, new_value);
}

fn part2_write(mem_key: usize, mem_value: usize,
    mask: &Vec<char>, mem_map: &mut HashMap<usize,usize>) {

  let mut new_key: usize = mem_key;
  let mut num_exes: usize = 0;
  for (n, bit) in mask.iter().rev().enumerate() {
    match bit { // Do the 1s and count the X's
      '1' => new_key = write_bit(new_key,n,1).unwrap(),
      'X' => num_exes += 1,
        _ => (),
    }
  }
  for num in 0..(1 << num_exes) { // 0 to num_exes to the power 2
    let mut cur_ex: usize = 0;
    for (n, &bit) in mask.iter().rev().enumerate() {
      if bit == 'X' {
        let digit = check_bit(num, cur_ex);
        new_key = write_bit(new_key, n, digit).unwrap();
        cur_ex += 1;
      }
    }
    mem_map.insert(new_key, mem_value);
  } 
}

pub fn advent() {

  let filename: String = "14.txt".to_string();
  let text = fs::read_to_string(filename).unwrap();

  let mut mask: Vec<char> = Vec::new();
  let mut mem_map: HashMap<usize,usize> = HashMap::new();

  for line in text.lines() {
    let inst: Vec<&str> = line.split_whitespace().collect();
    if inst[0] == "mask" {
      mask = inst[2].to_string().chars().collect();
    } else {
      let x: Vec<&str> = inst[0].split("]").collect();
      let mem_key: usize = x[0][4..].parse::<usize>().unwrap();
      let mem_value: usize = inst[2].parse::<usize>().unwrap();

      part2_write(mem_key, mem_value, &mask, &mut mem_map);
    }
  }
  let total: usize = mem_map.values().sum();
  println!("{:?}", total);
}