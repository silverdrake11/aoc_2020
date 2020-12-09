use std::fs;
use std::collections::HashSet;

const PREAMBLE: usize = 25;

fn check_num(num: &usize, set: &HashSet<usize>) -> bool {
  for preamble_num in set {
    if num >= preamble_num {
      if set.contains(&(num - preamble_num)) {
        return true
      }
    }
  }
  return false
}

fn get_answer(cur_idx: usize, window: usize, xmas: &Vec<usize>) -> usize{
  let slice = &xmas[cur_idx+1-window..cur_idx+1];
  return *slice.iter().min().unwrap() + *slice.iter().max().unwrap();
}

fn find_sum(num: usize, window: usize, xmas: &Vec<usize>) -> Option<usize> {
  let mut sum: usize = xmas[0..window].iter().sum();
  if num == sum {
    return Some(get_answer(window, window, xmas));
  }
  for i in window..xmas.len() {
    sum -= xmas[i-window];
    sum += xmas[i];
    if num == sum {
      return Some(get_answer(i, window, xmas));
    }
  }
  return None
}

pub fn day09() {

  let filename: String = "9.txt".to_string();

  let contents = fs::read_to_string(filename).unwrap();
  let mut set: HashSet<usize> = HashSet::new();
  let mut v: Vec<usize> = Vec::new();

  for (i,value) in contents.lines().enumerate() {
    let x = value.parse::<usize>().unwrap();
    v.push(x);
    if i < 25 {
      set.insert(x);
    }
  }

  for i in PREAMBLE..v.len() {
    let cur_num = v[i];
    let to_remove = v[i-PREAMBLE];

    if !check_num(&cur_num, &set) {

      println!("Part1) {}", cur_num);

      for window in 2..v.len() {
        if let Some(answer) = find_sum(cur_num, window, &v) {
          println!("Part2) {}", answer);
        }
      }
      break;
    }

    set.remove(&to_remove);
    set.insert(cur_num);
  }
}