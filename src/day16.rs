use std::fs;
use std::collections::HashSet;

fn parse_range(range: &str) -> (usize, usize) {
  let ranges: Vec<usize> = range.split('-')
      .map(|s| s.parse::<usize>().unwrap()).collect();
  return (ranges[0], ranges[1]);
}


pub fn advent() {

  let filename: String = "16.txt".to_string();
  let text = fs::read_to_string(filename).unwrap();

  let splitted: Vec<&str> = text.split("\n\n").collect();

  let mut valid_values: HashSet<usize> = HashSet::new(); 
  for rule in splitted[0].lines() {
    let rule_split: Vec<&str> = rule.split(':').collect();
    let range_split: Vec<&str> = rule_split[1].split_whitespace().collect();
    let (start,end) = parse_range(range_split[0]);
    (start..end).for_each(|i| {valid_values.insert(i);});
    let (start,end) = parse_range(range_split[2]);
    (start..end).for_each(|i| {valid_values.insert(i);});
  }

  let mut tickets: Vec<Vec<usize>> = Vec::new();
  let mut invalids: Vec<usize> = Vec::new();
  for ticket in splitted[2].lines() {
    if ticket.starts_with("nearby") {
      continue
    }
    let fields: Vec<usize> = ticket.split(",")
        .map(|s| s.parse::<usize>().unwrap()).collect();
    for field in &fields {
      if !valid_values.contains(field) {
        invalids.push(*field);
      }
    }
    tickets.push(fields);
  }

  let sum: usize = invalids.iter().sum();
  println!("{:?}", sum);

}