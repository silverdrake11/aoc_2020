use std::fs;
use std::collections::HashSet;
use itertools::enumerate;


pub fn day08() {

  let filename: String = "8.txt".to_string();

  let contents = fs::read_to_string(filename).unwrap();

  let mut prog: Vec<(&str,i16)> = Vec::new();
  let mut flips: Vec<usize> = Vec::new();

  for (i, line) in enumerate(contents.lines()) {
    let inst: Vec<&str> = line.split_whitespace().collect();
    let arg: i16 = inst[1].parse::<i16>().unwrap();
    if inst[0] == "jmp" || inst[0] == "nop" {
      flips.push(i);
    }
    prog.push((inst[0], arg));
  }

  println!("{:?}", flips);

  for j in flips {

    let mut i: i16 = 0;
    let mut acc: i16 = 0;
    let mut set: HashSet<i16> = HashSet::new();

    loop {

      if (i as usize) >= prog.len() {
        println!("{:?}", acc);
        break;
      }

      let mut op = prog[i as usize].0;
      let arg = prog[i as usize].1;
      if set.contains(&i) { // Part 1
        break;
      }
      set.insert(i);

      if (i as usize) == j {
        op = match op {
          "jmp" => "nop",
          "nop" => "jmp",
          _ => op,
        };
      }

      match op {
        "acc" => acc += arg,
        "jmp" => i += arg - 1,
        _ => (),
      }

      i += 1;

    }
 }
}