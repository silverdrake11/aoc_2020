use std::fs;


const TREE: u8 = 35;

struct Point {
  y: usize,
  x: usize
}

struct Map<'a> {
  ylen: usize,
  xlen: usize,
  map: Vec<&'a [u8]>
}

impl Map<'_> {

  pub fn new(ylen: usize, xlen: usize, map: Vec<&[u8]>) -> Map {
    return Map {ylen: ylen, xlen: xlen, map: map};
  }

  pub fn is_tree(&self, point: Point) -> Result<bool, String> {
    
    let x: usize = point.x % self.xlen;
    let y: usize = point.y;

    if y >= self.ylen {
      return Err("Out of bounds!".to_string());
    }

    if self.map[y][x] == TREE {
      return Ok(true);
    } else {
      return Ok(false);
    }
  }

}

fn count_trees(slope: &Point, map: &Map) -> usize {

  let mut y: usize = 0;
  let mut x: usize = 0;
  let mut num_trees: usize = 0;

  loop {
    match map.is_tree(Point{y:y, x:x}) {

      Ok(is_tree) => {

        y += slope.y;
        x += slope.x;

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

  let mut map_bytes: Vec<&[u8]> = Vec::new();
  for value in text.lines() {
    map_bytes.push(value.as_bytes());
  }

  let ylen = map_bytes.len();
  let xlen = map_bytes[0].len();

  let map_obj = Map::new(ylen, xlen, map_bytes);

  let mut slopes: Vec<Point> = Vec::new();
  slopes.push(Point {y: 1, x:3});

  // Part 1
  println!("Part 1) {}", count_trees(&slopes[0], &map_obj));

  // Part 2
  slopes.push(Point {y: 1, x:1});
  slopes.push(Point {y: 1, x:5});
  slopes.push(Point {y: 1, x:7});
  slopes.push(Point {y: 2, x:1});
  let mut tree_product = 1;
  for slope in &slopes {
    tree_product *= count_trees(slope, &map_obj);
  }

  println!("Part 2) {}", tree_product);

}

