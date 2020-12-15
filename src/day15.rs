
pub fn day15() {

  let v: Vec<usize> = vec![1,0,18,10,19,6];
  let input_len = v.len();
  let max: usize = 30000000;
  let mut nums_spoken: Vec<usize> = vec![0; max];
  for i in 0..input_len {
    nums_spoken[v[i]] = i+1;
  }
  let mut last: usize = 0;
  for n in (input_len+1)..max {
    let temp: usize = 
      if nums_spoken[last] > 0 {
        n - nums_spoken[last]
      } else {
        0
      };
    nums_spoken[last] = n;
    last = temp;
  }
  println!("{}", last);
}