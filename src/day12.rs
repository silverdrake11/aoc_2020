use std::fs;

#[derive(PartialEq,Clone,Copy,Debug)]
 enum Dir {
    N,
    E,
    S,
    W,
}

fn get_dir(cur: Dir, hand: char, degree: usize) -> Dir {
  let diff = degree / 90;
  let v: Vec<Dir> = vec![Dir::N, Dir::E, Dir::S, Dir::W];
  let dirs: Vec<Dir> = match hand {
    'R' => v,
    _ => v.into_iter().rev().collect()
  };
  let cur_idx = dirs.iter().position(|&s| s == cur).unwrap();
  let new_idx = (cur_idx + diff) % 4;
  return dirs[new_idx];
}

fn get_point(dir: Dir, num: usize, point: (usize, usize)) -> (usize, usize) {
  let (y,x) = point;
  match dir {
    Dir::N => (y+num,x),
    Dir::S => (y-num,x),
    Dir::E => (y,x+num),
    _ => (y,x-num),
  }
}

fn diff(a: usize, b: usize) -> usize {
  if a > b {
    return a - b
  } else {
    return b - a
  }
}

fn man_dist(p1: (usize, usize), p2: (usize, usize)) -> usize {
  return diff(p1.0, p2.0) + diff(p1.1, p2.1) 
}

pub fn day12() {

  let filename: String = "12.txt".to_string();

  let text = fs::read_to_string(filename).unwrap();
  //let (y,x): (usize,usize) = (1000,1000);

  /*
  println!("{:?}", hello(Dir::E, 'L', 90));
  println!("{:?}", hello(Dir::E, 'L', 180));
  println!("{:?}", hello(Dir::E, 'L', 270));
  println!("{:?}", hello(Dir::E, 'R', 90));*/

  let mut cur_dir = Dir::E;
  let start: (usize, usize) = (10000,10000);
  let mut cur_point: (usize, usize) = start;

  for line in text.lines() {
    let dir = line.chars().nth(0).unwrap();
    let &num = &line[1..].parse::<usize>().unwrap();
    match dir {
      'N' => cur_point = get_point(Dir::N, num, cur_point),
      'E' => cur_point = get_point(Dir::E, num, cur_point),
      'S' => cur_point = get_point(Dir::S, num, cur_point),
      'W' => cur_point = get_point(Dir::W, num, cur_point),
      'L' => cur_dir = get_dir(cur_dir, 'L', num),
      'R' => cur_dir = get_dir(cur_dir, 'R', num),
      'F' => cur_point = get_point(cur_dir, num, cur_point),
      _ => (),
    };
    println!("{:?} {:?}", cur_dir, cur_point);
  }

  println!("{:?}", man_dist(start,cur_point));


}