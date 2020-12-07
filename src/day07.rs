use std::fs;
use std::collections::HashMap;

fn recursive(key: &String, tree: &HashMap<String,HashMap<String,usize>>) -> usize{

  //if key == "shiny gold" {
  // return 1}

  let mut count: usize = 1;
  for (child_key, child_value) in &tree[key] {
    count += child_value * recursive(child_key, tree);
  }

  return count;

}

pub fn day07() {

  let filename: String = "7.txt".to_string();

  let contents = fs::read_to_string(filename).unwrap();
  let mut tree: HashMap<String,HashMap<String,usize>> = HashMap::new();

  for line in contents.lines() {

    let words: Vec<&str> = line.split_whitespace().collect();
    
    let mut leaf: HashMap<String, usize> = HashMap::new();

    let offset: usize = 4;
    if words.len() >= offset * 2{

      let mut i: usize = offset;
      while i <= words.len() - offset {
        words[i];
        let key: String = [words[i+1], words[i+2]].join(" ");
        let value: usize = words[i].parse::<usize>().unwrap();
        leaf.insert(key, value);
        i += offset;
      }
      
    }

    tree.insert([words[0], words[1]].join(" ").to_string(), leaf);
  }

  let mut count: usize = 0;
  for key in tree.keys() {
    let answer: usize = recursive(key, &tree);
    
    if answer > 0 {
      if key == "shiny gold" {
        println!("{} {}", key, answer-1);
        count += 1;
      }
    }
  }
  println!("{}", count);
}