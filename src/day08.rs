use std::fs;
use std::collections::HashSet;
use itertools::enumerate;

#[derive(PartialEq,Clone,Copy)]
 enum Op {
    Jmp,
    Nop,
    Acc,
    Err,
}

pub fn advent() {

  let filename: String = "8.txt".to_string();

  let contents = fs::read_to_string(filename).unwrap();

  let mut prog: Vec<(Op,i16)> = Vec::new();
  let mut flips: Vec<usize> = Vec::new();

  for (i, line) in enumerate(contents.lines()) {
    let inst: Vec<&str> = line.split_whitespace().collect();
    let arg: i16 = inst[1].parse::<i16>().unwrap();
    let op = match inst[0] {
      "acc" => Op::Acc,
      "jmp" => Op::Jmp,
      "nop" => Op::Nop,
      _ => Op::Err,
    };
    if op == Op::Jmp || op == Op::Nop {
      flips.push(i);
    }
    prog.push((op, arg));
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

      let mut op = prog[i as usize].0.clone();
      let arg = prog[i as usize].1;
      if set.contains(&i) { // Part 1
        break;
      }
      set.insert(i);

      if (i as usize) == j {
        op = match op {
          Op::Jmp => Op::Nop,
          Op::Nop => Op::Jmp,
          _ => op,
        };
      }

      match op {
        Op::Acc => acc += arg,
        Op::Jmp => i += arg - 1,
        _ => (),
      }

      i += 1;

    }
 }
}