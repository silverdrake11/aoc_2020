use std::fs;
use std::collections::HashMap;
use std::collections::HashSet;

fn get_seat(point: (usize, usize), slope: (i32, i32), rows: &Vec<Vec<char>>) -> Option<char> {
  let ymax = rows.len();
  let xmax = rows[0].len();
  let (y,x) = point;
  let (dy,dx) = slope;
  if dx + (x as i32) < 0 {
    return None;
  }
  if dy + (y as i32) < 0 {
    return None;
  }
  let yadj = (y as i32 + dy) as usize;
  let xadj = (x as i32 + dx) as usize;
  if yadj >= ymax || xadj >= xmax {
    return None;
  }
  return Some(rows[yadj][xadj]);
}

fn get_changes_part1(point: (usize, usize), rows: &Vec<Vec<char>>) -> Option<char>{
  let (y, x) = point;

  let seat = rows[y][x];

  if seat == '.' {
    return None
  }

  let mut occupied: usize = 0;
  let dirs = [(0,1),(1,0),(1,1),(0,-1),(-1,0),(-1,-1),(-1,1),(1,-1)];
  for &dir in &dirs {
    if let Some(seat_adj) = get_seat(point, dir, &rows) {
      if seat_adj == '#' {
        occupied += 1;
      }
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

fn get_changes_part2(point: (usize, usize), rows: &Vec<Vec<char>>) -> Option<char> {
  let (y, x) = point;

  let seat = rows[y][x];

  if seat == '.' {
    return None
  }

  let mut occupied: usize = 0;
  let mut dirs: HashSet<(i32,i32)> = vec![(0,1),(1,0),(1,1),(0,-1),(-1,0),(-1,-1),(-1,1),(1,-1)].into_iter().collect();
  let mut r: i32 = 1;
  while dirs.len() > 0 {
    let mut to_remove: Vec<(i32,i32)> = Vec::new();
    for &dir in &dirs {
      let dy = dir.0 * r;
      let dx = dir.1 * r;
      if let Some(seat_adj) = get_seat(point, (dy,dx), &rows) {
        if seat_adj == '.' {
          continue
        }
        if seat_adj == '#' {
          occupied += 1;
          if seat == '#' {
            if occupied >= 5 {
              return Some('L');
            }
          } else { // If L
            return None
          }
        }
      }
      to_remove.push(dir);
    }
    for el in to_remove {
      dirs.remove(&el);
    }
    r += 1;
  }
  if seat == 'L' && occupied == 0 {
    return Some('#')
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
        if let Some(seat) = get_changes_part2((y,x), &rows) {
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