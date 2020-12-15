use std::fs;

pub fn advent() {

  let filename: String = "13.txt".to_string();
  let text = fs::read_to_string(filename).unwrap();

  let start_time = text.lines().nth(0).unwrap().parse::<usize>().unwrap();
  let splitted: Vec<&str> = text.lines().nth(1).unwrap().split(',').collect();
  let mut buses: Vec<usize> = Vec::new();
  for &note in &splitted {
    if note != "x" {
      buses.push(note.parse::<usize>().unwrap());
    } else {
      buses.push(0);
    }
  }

  'outer: for t in 0..10000 {
    let time = t + start_time;
    for &bus in &buses {
      if bus != 0 { // x is 0
        if time % bus == 0 {
          println!("Part1) {}", (time-start_time)*bus);
          break 'outer;
        }
      }
    }
  }

  let mut timestamp: usize = 1;
  let mut wait_time: usize = 1;
  for (bus_num, &bus_minutes) in buses.iter().enumerate() {
    if bus_minutes == 0 { // Skip this as it's the 'x'
      continue
    }
    while (timestamp + bus_num) % bus_minutes != 0 {
      timestamp += wait_time;
    }
    wait_time *= bus_minutes;
  }
  println!("Part2) {}", timestamp);
}