use std::fs;

fn rotate(point: (i32, i32), degree: i32, hand: char) -> (i32, i32) {
  let rotations = degree / 90;
  let mut rotated: (i32, i32) = point;
  for _ in 0..rotations {
    if hand == 'L' {
      rotated = (rotated.1, -rotated.0);
    } else {
      rotated = (-rotated.1, rotated.0);
    }
  }
  return rotated
}

pub fn day12() {

  let filename: String = "12.txt".to_string();

  let text = fs::read_to_string(filename).unwrap();

  let start = (0,0);  
  let mut ship_point = start;
  let mut cur_point = (1,10);

  for line in text.lines() {
    let dir = line.chars().nth(0).unwrap();
    let &num = &line[1..].parse::<i32>().unwrap();
    let (y,x) = cur_point;
    match dir {
      'N' => cur_point = (y+num,x),
      'E' => cur_point = (y,x+num),
      'S' => cur_point = (y-num,x),
      'W' => cur_point = (y,x-num),
      'L' => cur_point = rotate(cur_point, num, dir),
      'R' => cur_point = rotate(cur_point, num, dir),
      'F' => ship_point = (num*y+ship_point.0,num*x+ship_point.1),
      _ => (),
    };
  }
  println!("Part 2) {}", ship_point.0.abs() + ship_point.1.abs()); // Part 1 is in previous commit
}