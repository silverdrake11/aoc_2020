use std::collections::HashMap;

pub fn day15() {

  let v: Vec<usize> = vec![1,0,18,10,19,6];
  let mut nums_spoken: HashMap<usize,usize> = HashMap::new();
  //v.iter().enumerate().map(|(i,&n)| nums_spoken.insert(n,i)).collect();
  for i in 0..v.len() {
    nums_spoken.insert(v[i],i+1);
  }
  let mut max: usize = 0;
  let mut last: usize = 0;
  for n in (v.len()+1)..30000000 {
    let mut temp: usize =  0;
    if nums_spoken.contains_key(&last) {
      temp = n - nums_spoken[&last];
    } else {
      temp = 0;
    }
    nums_spoken.insert(last,n);
    last = temp;
    if last > max {
      max = last;
    }
    //println!("{}", last);
  }
  println!("{}", max);
}