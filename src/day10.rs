use std::fs;
use std::collections::HashSet;

#[derive(PartialEq,Clone,Copy)]
 enum Op {
    Jmp,
    Nop,
    Acc,
    Err,
}

fn run(prog: &Vec<(Op,i16)>) {

  let mut i: i16 = 0;
  let mut acc: i16 = 0;
  let mut set: HashSet<i16> = HashSet::new();

  loop {

    if (i as usize) >= prog.len() {
      println!("{:?}", acc);
      break
    }

    let op = prog[i as usize].0.clone();
    let arg = prog[i as usize].1;
    if set.contains(&i) {
      println!("{:?}", acc);
      break
    }
    set.insert(i);

    match op {
      Op::Acc => acc += arg,
      Op::Jmp => i += arg - 1,
      _ => (),
    }
    i += 1;
  }
}

pub fn day10() {

  let filename: String = "10.txt".to_string();

  let contents = fs::read_to_string(filename).unwrap();

  let mut prog: Vec<(Op,i16)> = Vec::new();

  for line in contents.lines() {
    let inst: Vec<&str> = line.split_whitespace().collect();
    let arg: i16 = inst[1].parse::<i16>().unwrap();
    let op = match inst[0] {
      "acc" => Op::Acc,
      "jmp" => Op::Jmp,
      "nop" => Op::Nop,
      _ => Op::Err,
    };
    prog.push((op, arg));
  }

  run(&prog);
 
}