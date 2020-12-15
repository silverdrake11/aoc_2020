
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
    let when_spoken = nums_spoken[last];
    nums_spoken[last] = n;
    last = 
      if when_spoken > 0 { // If it was spoken
        n - when_spoken
      } else {
        0
      };
  }
  println!("{}", last);
}