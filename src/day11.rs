use std::fs;
use std::collections::HashMap;

fn get_changes(point: (usize, usize), rows: &Vec<Vec<char>>) -> Option<char>{
  let (y, x) = point;

  let seat = rows[y][x];

  if seat == '.' {
    return None
  }

  let ymax = rows.len();
  let xmax = rows[0].len();

  let mut occupied: usize = 0;
  let dirs = [(0,1),(1,0),(1,1),(0,-1),(-1,0),(-1,-1),(-1,1),(1,-1)];
  for dir in &dirs {
    let (dy, dx) = dir;
    if *dx < 0 && x == 0 {
      continue;
    }
    if *dy < 0 && y == 0 {
      continue;
    }
    let yadj = (y as i32 + dy) as usize;
    let xadj = (x as i32 + dx) as usize;

    if yadj >= ymax || xadj >= xmax {
      continue;
    }
    if rows[yadj][xadj] == '#' {
      occupied += 1;
    }
  }

  if seat == 'L' && occupied == 0 {
    return Some('#')
  }

  if seat == '#' && occupied >= 4 {
    return Some('L')
  }

  return None
}

pub fn day11() {

  let filename: String = "11.txt".to_string();

  let text = fs::read_to_string(filename).unwrap();

  let mut rows: Vec<Vec<char>> = Vec::new();
  for line in text.lines() {
    rows.push(line.chars().collect());
  }

  loop {
    let mut changes: HashMap<(usize,usize),char> = HashMap::new();
    for y in 0..rows.len() {
      for x in 0..rows[0].len() {
        if let Some(seat) = get_changes((y,x), &rows) {
          changes.insert((y,x),seat);
        }
      }
    }

    if changes.len() == 0 {
      break;
    }

    for (&point, &seat) in &changes {
      let (y,x) = point;
      rows[y][x] = seat;
    }
  }

  let mut occupied = 0;
  for y in 0..rows.len() {
    for x in 0..rows[0].len() {
      if rows[y][x] == '#' {
        occupied += 1;
      }
    }
  }

  println!("{:?}", occupied);
}