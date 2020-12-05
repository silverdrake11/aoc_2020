use std::fs;

fn get_seat_helper(part: &[char], max: usize, left_char: char) -> usize  {

  let mut left: usize = 0;
  let mut right: usize = max;

  for &c in part {
    let mid = left + (right - left) / 2;
    if c == left_char {
      right = mid;
    } else {
      left = mid + 1;
    }
  }
  return left;
}

fn get_seat(part: &Vec<char>) -> usize {
  let row = get_seat_helper(&part[..7], 127, 'F');
  let col = get_seat_helper(&part[7..], 7, 'L');
  return row * 8 + col;
}

pub fn day05() {

  let filename: String = "5.txt".to_string();
  let text = fs::read_to_string(filename).unwrap();

  let mut seats: Vec<usize> = Vec::new();

  for line in text.lines() {
    let part: Vec<char> = line.chars().collect();
    let seat_id = get_seat(&part);
    seats.push(seat_id);
  }

  seats.sort();

  println!("Part 1) {}", seats.last().unwrap());

  for (i, &seat_id) in seats.iter().enumerate() {
    if i + seats[0] != seat_id {
      println!("Part 2) {}", seat_id - 1);
      break;
    }
  }
}