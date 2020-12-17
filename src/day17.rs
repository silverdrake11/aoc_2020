use std::fs;
use std::collections::HashSet;


fn get_neighbors(p: (i32,i32,i32)) -> Vec<(i32,i32,i32)> {
  let mut neighbors: Vec<(i32,i32,i32)> = Vec::new();
  for x in &[-1,0,1] {
    for y in &[-1,0,1] {
      for z in &[-1,0,1] {
        neighbors.push((p.0+x,p.1+y,p.2+z));
      }
    }
  }
  return neighbors
}

fn get_inactive(active_cubes: &HashSet<(i32, i32, i32)>) -> Vec<(i32,i32,i32)>{

  let mut inactive: HashSet<(i32,i32,i32)> = HashSet::new();
  for &active in active_cubes {
    for neighbor in get_neighbors(active) {
      if !active_cubes.contains(&neighbor) {
        inactive.insert(neighbor);
      }
    }
  }
  return inactive.into_iter().collect()
}

fn count_active(points: &Vec<(i32,i32,i32)>, active_cubes: &HashSet<(i32, i32, i32)>) -> usize {
  let mut count = 0;
  for neighbor in points {
    if active_cubes.contains(&neighbor) {
      count += 1;
      if count > 3 {
        return count
      }
    }
  }
  return count;
}

fn print(active_cubes: &HashSet<(i32, i32, i32)>) {
  for z in -1..4 {
    println!("z={}", z-1);
    for y in -10..10 {
      for x in -10..10 {
          if active_cubes.contains(&(y,x,z)) {
            print!("#");
          } else {
            print!(".");
          }
      }
      println!("");
    }
  }
}

fn cycle(active_cubes: &mut HashSet<(i32, i32, i32)>) {
  let mut to_delete: Vec<(i32,i32,i32)> = Vec::new();
  for &active in active_cubes.iter() {
    let count = count_active(&get_neighbors(active), &active_cubes);
    if count != 2 && count != 3 {
      to_delete.push(active);
    }
  }
  let mut to_add: Vec<(i32,i32,i32)> = Vec::new();
  for inactive in get_inactive(&active_cubes) {
    let count = count_active(&get_neighbors(inactive), &active_cubes);
    if count == 3 {
      to_add.push(inactive);
    }
  }
  for d in to_delete {
    active_cubes.remove(&d);
  }
  for a in to_add {
    active_cubes.insert(a);
  }
}

pub fn advent() {

  let filename: String = "17.txt".to_string();
  let text = fs::read_to_string(filename).unwrap();


  let mut init: Vec<Vec<char>> = Vec::new();
  for line in text.lines() {
    init.push(line.chars().collect());
  }

  let mut active_cubes: HashSet<(i32,i32,i32)> = HashSet::new();
  for y in 0..init.len() {
    for x in 0..init[0].len() {
      if init[y][x] == '#' {
        active_cubes.insert((y as i32,x as i32,1));
      }
    }
  }

  for _ in 0..6 {
    cycle(&mut active_cubes);
    //print(&active_cubes);
    println!("{:?}", active_cubes.len());
  }

  //cycle(&mut active_cubes);
  //println!("{:?}", active_cubes);
  //cycle((2,2,1), &mut active_cubes);
  //println!("{:?}", get_neighbors((2,2,1)));
  //println!("{:?}", get_inactive(&active_cubes));


}