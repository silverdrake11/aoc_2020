use std::fs;


pub fn advent() {

  let filename: String = "10.txt".to_string();

  let contents = fs::read_to_string(filename).unwrap();

  let mut adapters: Vec<usize> = Vec::new();
  adapters.push(0);

  for line in contents.lines() {
    let x = line.parse::<usize>().unwrap();
    adapters.push(x);
  }

  adapters.sort();
  adapters.push(adapters.iter().last().unwrap() + 3);

  let mut count1 = 0;
  let mut count3 = 0;

  let mut diffs: Vec<usize> = Vec::new();

  for slice in adapters.windows(2) {
    let diff = slice[1] - slice[0];
    println!("{} {}", diff, slice[0]);
    match diff {
      1 => count1 += 1,
      3 => count3 += 1,
      _ => println!("Error!"),
    }
    diffs.push(diff);
  }

  let mut count = 0;
  for slice in diffs.windows(3) {
    if slice[0] == 1 && slice[1] == 1 {
      if slice[2] == 1 || slice[2] == 3 {
        count += 1;
      }
    }
  }

  let n: usize = 2;
  println!("{} {}", count1 * count3, n.pow(count));
 
}