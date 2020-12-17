use std::fs;
use std::collections::HashSet;
use std::collections::HashMap;


fn parse_range(range: &str) -> (usize, usize) {
  let ranges: Vec<usize> = range.split('-')
      .map(|s| s.parse::<usize>().unwrap()).collect();
  return (ranges[0], ranges[1]);
}

//fn check_ticket(fields: &Vec<usize>, invalids: &HashSet<usize>)

fn check_field(field_id: usize, valid_nums: &HashSet<usize>, tickets: &Vec<Vec<usize>>) -> bool {
  for ticket_id in 0..tickets.len() {
    let value = tickets[ticket_id][field_id];
    if !valid_nums.contains(&value) {
      return false;
    }
  }
  return true;
}



pub fn advent() {

  let filename: String = "16.txt".to_string();
  let text = fs::read_to_string(filename).unwrap();

  let splitted: Vec<&str> = text.split("\n\n").collect();

  let mut valid_values: HashSet<usize> = HashSet::new();
  let mut fields_map: HashMap<usize,HashSet<usize>> = HashMap::new();
  let mut fields: Vec<String> = Vec::new();
  for rule in splitted[0].lines() {
    let rule_split: Vec<&str> = rule.split(':').collect();
    let range_split: Vec<&str> = rule_split[1].split_whitespace().collect();

    let mut cur_values: HashSet<usize> = HashSet::new();
    let (start,end) = parse_range(range_split[0]);
    (start..end+1).for_each(|i| {valid_values.insert(i); cur_values.insert(i);});
    let (start,end) = parse_range(range_split[2]);
    (start..end+1).for_each(|i| {valid_values.insert(i); cur_values.insert(i);});
    fields_map.insert(fields.len(), cur_values);
    fields.push(rule_split[0].to_string());
  }
  println!("{:?}", fields);

  let mut tickets: Vec<Vec<usize>> = Vec::new();
  for ticket in splitted[2].lines() {
    if ticket.starts_with("nearby") {
      continue
    }
    let fields: Vec<usize> = ticket.split(",")
        .map(|s| s.parse::<usize>().unwrap()).collect();
    if fields.iter().all(|v| valid_values.contains(v)) {
      tickets.push(fields);
    }
  }

  let my_ticket: Vec<usize> = splitted[1].lines().nth(1).unwrap().split(",")
        .map(|s| s.parse::<usize>().unwrap()).collect();
  println!("{:?}", my_ticket);
  println!("{:?}", fields);

  let num_fields = fields.len();
  let mut found: HashMap <usize, usize> = HashMap::new();
  let mut elim_matrix: Vec<Vec<bool>> = Vec::new();
  for field_id in 0..num_fields {
    let mut rules_test: Vec<bool> = Vec::new();
    for rule_id in 0..num_fields {
      let status: bool;
      if !check_field(field_id, &fields_map[&rule_id], &tickets) {
        status = false;
      } else {
        status = true;
      }
      rules_test.push(status);
    }
    elim_matrix.push(rules_test);
  }

  println!("{:?}", elim_matrix);

  while found.len() < num_fields {
  for field_id in 0..num_fields {
    let mut count: usize = 0;
    let mut idx: usize = 0;
    if found.contains_key(&field_id) {
      continue;
    }
    for rule_id in 0..num_fields {
       if elim_matrix[field_id][rule_id] == true {
        count += 1;
        idx = rule_id;
       }
    }
    if count == 1 {
      for field_id in 0..num_fields {
        elim_matrix[field_id][idx] = false;
      }
      found.insert(field_id, idx);
    }
  }
  }

  println!("{:?}", elim_matrix);
  println!("{:?}", found);

  let mut answer = 1;
  for (field_id, rule_id) in found {
    let field_value = my_ticket[field_id];
    println!("{:?} {}", fields[rule_id], field_value);
    if fields[rule_id].starts_with("departure") {
      answer *= field_value;
      println!("{:?}", answer);
    }
  }
  println!("{:?}", answer);

}