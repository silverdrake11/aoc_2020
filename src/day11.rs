use std::fs;
use std::collections::HashMap;

fn get_seat(point: usize, slope: (i32, i32), length: usize, width: usize) -> Option<usize> {
  let y = point / width;
  let x = point % width;
  let (dy,dx) = slope;
  if dx + (x as i32) < 0 {
    return None;
  }
  if dy + (y as i32) < 0 {
    return None;
  }
  let yadj = (y as i32 + dy) as usize;
  let xadj = (x as i32 + dx) as usize;
  if yadj >= length || xadj >= width {
    return None;
  }
  return Some(xadj + width*yadj);
}

fn get_changes_part1(point: usize, width: usize, seats: &Vec<char>) -> Option<char>{

  let seat = seats[point];
  if seat == '.' {
    return None
  }
  let length = seats.len() / width;

  let mut occupied: usize = 0;
  let dirs = [(0,1),(1,0),(1,1),(0,-1),(-1,0),(-1,-1),(-1,1),(1,-1)];
  for &dir in &dirs {
    if let Some(seat_adj) = get_seat(point, dir, length, width){
      if seats[seat_adj] == '#' {
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

fn get_changes_part2(point: usize, seats: &Vec<char>, adj_points: &Vec<usize>) -> Option<char> {

  if adj_points.len() == 0 {
      return None
  }

  let seat = seats[point];
  let mut occupied: usize = 0;

  if seat == '#' {
    for seat_adj in adj_points.iter().map(|&i| seats[i]) {
      if seat_adj == '#' {
        occupied += 1;
        if occupied >= 5 {
          return Some('L');
        }
      }
    }    
  } else { // L
    if adj_points.iter().all(|&i| seats[i] == 'L') {
      return Some('#');
    } 
  }
  return None
}

fn get_adj_seats(seat: usize, width: usize, seats: &Vec<char>) -> Vec<usize> {

  let mut adj_seats: Vec<usize> = Vec::new();
  if seats[seat] == '.' {
    return adj_seats
  }

  let length = seats.len() / width;

  let dirs = [(0,1),(1,0),(1,1),(0,-1),(-1,0),(-1,-1),(-1,1),(1,-1)];
  for &dir in &dirs {
    let mut mag: i32 = 1;
    let mut dy = dir.0;
    let mut dx = dir.1;

    while let Some(adj_seat) = get_seat(seat, (dy,dx), length, width) {
      mag += 1;
      dy = dir.0 * mag;
      dx = dir.1 * mag;
      if seats[adj_seat] != '.' {
        adj_seats.push(adj_seat);
        break;
      }
    }
  }
  return adj_seats;
}

pub fn day11() {

  let filename: String = "11.txt".to_string();

  let text = fs::read_to_string(filename).unwrap();

  let mut seats: Vec<char> = Vec::new();
  let mut width = 0;
  for line in text.lines() {
    width = line.len();
    for c in line.chars() {
      seats.push(c);
    }
  }

  let mut adj_map: Vec<Vec<usize>> = Vec::new();
  for i in 0..seats.len() {
    let adj_seats = get_adj_seats(i, width, &seats);
    adj_map.push(adj_seats);
  }

  loop {
    let mut changes: HashMap<usize,char> = HashMap::new();
    for i in 0..seats.len() {
      let adj_points = &adj_map[i];

      //if let Some(seat) = get_changes_part1(i, width, &seats) { // Part 1
      if let Some(seat) = get_changes_part2(i, &seats, &adj_points) { // Part 2
        changes.insert(i,seat);
      }
    }
    if changes.len() == 0 {
      break;
    }
    for (&point, &seat) in &changes {
      seats[point] = seat;
    }
  }
  let occupied = seats.iter().filter(|&n| *n == '#').count();
  println!("{}", occupied);
}