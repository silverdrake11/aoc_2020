use std::fs;
use std::collections::HashSet;
use itertools::enumerate;

fn count_all_yes(group: &Vec<HashSet<char>>) -> usize { // For part 2
  if group.is_empty() {
    return 0
  }
  
  // Find person who answered least questions
  let mut min_person: usize = 0;
  let mut min_size: usize = group[min_person].len();
  for (i, person) in enumerate(group) {
    if person.len() < min_size {
      min_person = i;
      min_size = person.len();
    }
  }
  let min_set = group[min_person].clone();

  // Eliminate any questions that weren't always answered
  let mut to_remove: HashSet<&char> = HashSet::new();
  for person in group {
    for c in min_set.iter().filter(|c| !person.contains(c)) {
      to_remove.insert(&c);
    }
  }
  return min_set.len() - to_remove.len();
}

pub fn day06() {

  let filename: String = "6.txt".to_string();

  let contents = fs::read_to_string(filename).unwrap();

  let mut set: HashSet<char> = HashSet::new();
  let mut group: Vec<HashSet<char>> = Vec::new();

  let mut part1: usize = 0;
  let mut part2: usize = 0;

  for line in contents.lines() {

    for item in line.split(" ") {

      if item == "" {

        set.clear();

        part2 += count_all_yes(&group);

        group.clear();

      } else {

      let person: HashSet<char> = item.chars().collect();

      for c in &person {
        if !set.contains(c) {
          set.insert(*c);
          part1 += 1;
        }
      }

      group.push(person);

      }
    }
  }
  println!("Part 1) {}", part1);
  println!("Part 2) {}", part2);
}