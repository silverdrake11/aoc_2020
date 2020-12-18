use std::fs;

const RADIX: u32 = 10;

fn simplify_bracket(ex: &[Vec<char>]) -> Vec<Vec<char>> {
  let mut new: Vec<Vec<char>> = Vec::new();
  let mut first: Vec<char> = ex[0].clone();
  first.remove(0);
  new.push(first);
  let mut count = 0;
  for (i,op) in ex.iter().enumerate() {
    for &c in op {
        if c == '(' {
          count += 1;
        }
        if c == ')' {
          count -= 1;
          if count == 0 {
            let mut last: Vec<char> = ex[i].clone();
            last.pop();
            new.push(last);
            return new;
          }
        }
      }
    if i != 0 {
      new.push(op.clone());
    }
  }
  return new;
}

fn hello(ex: &Vec<Vec<char>>) -> usize {
  let mut total: usize = 0;
  let mut cursor: usize = 0;
  while cursor < ex.len() {
    let cur_char = ex[cursor][0];
    let next_char = ex[cursor+1][0];

    if cur_char == '*' || cur_char == '+' {
      let next_exp: usize;
      let next_len: usize;
      if next_char == '(' {
        let v = simplify_bracket(&ex[cursor+1..]);
        next_len = v.len()+1;
        next_exp = hello(&v);
      } else {
        next_exp = next_char.to_digit(RADIX).unwrap() as usize;
        next_len = 2;
      }
      if cur_char == '*' {
        total *= next_exp;
      } else {
        total += next_exp;
      }
      cursor += next_len;
    } else if cur_char == '(' {
      let cur_ex = simplify_bracket(&ex[cursor..]);
      cursor += cur_ex.len();
      total = hello(&cur_ex);
    } else { // Note it will only be a digit at the beginning of expression
      total = cur_char.to_digit(RADIX).unwrap() as usize;
      cursor += 1;
    }
  }
  return total;
}

pub fn advent() {

  let filename: String = "18.txt".to_string();
  let text = fs::read_to_string(filename).unwrap();
  let mut values: usize = 0;
  for line in text.lines() {
    let splitted: Vec<&str> = line.split_whitespace().collect();
    let mut init: Vec<Vec<char>> = Vec::new();
    for item in splitted {
      init.push(item.chars().collect());
    }
    values += hello(&init);
  }
  println!("{:?}", values);
}