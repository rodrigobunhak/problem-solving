fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
  let mut k: i32 = 0;
  for i in 0..nums.len() {
    if nums[i] != val {
      nums[k as usize] = nums[i];
      k += 1;
    }
  }
  k
}

fn main() {
  let mut nums = vec![0,1,2,2,3,0,4,2];
  let k = remove_element(&mut nums, 2);
  println!("{}", k);
}