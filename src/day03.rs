use std::fs;


fn is_tree(point: (usize, usize), map: &Vec<Vec<char>>) -> Result<bool, String> {
  
  let (y, x) = point;
  let x = x % map[0].len();

  if y >= map.len() {
    return Err("Out of bounds!".to_string());
  }

  if map[y][x] == '#' {
    return Ok(true);
  } else {
    return Ok(false);
  }
}


fn count_trees(slope: (usize, usize), map: &Vec<Vec<char>>) -> usize {

  let mut y: usize = 0;
  let mut x: usize = 0;
  let (dy, dx) = slope;
  let mut num_trees: usize = 0;

  loop {

    let mut cur_point = (y, x);

    match is_tree(cur_point, &map) {

      Ok(is_tree) => {

        y += dy;
        x += dx;

        if is_tree {
          num_trees += 1;
        }
      },

      Err(_) => break,
    };
  }

  return num_trees;
}


pub fn day03() {

  let filename: String = "3.txt".to_string();
  let text = fs::read_to_string(filename).unwrap();

  let mut map: Vec<Vec<char>> = Vec::new();
  for value in text.lines() {
    map.push(value.chars().collect());
  }

  // Part 1
  println!("Part 1) {}", count_trees((1,3), &map));

  // Part 2
  let slopes = [(1, 1),(1, 5),(1, 7),(2, 1),(1,3)];
  let mut tree_product = 1;
  for slope in &slopes {
    tree_product *= count_trees(*slope, &map);
  }

  println!("Part 2) {}", tree_product);

}