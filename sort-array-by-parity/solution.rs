fn sort_array_by_parity(nums: &mut Vec<i32>) {
  let mut left_pointer: usize = 0;
  let mut right_pointer: usize = nums.len() - 1;
  while left_pointer < right_pointer {
    if nums[left_pointer] % 2 == 0 {
      left_pointer += 1;
    } else if nums[right_pointer] % 2 != 0 {
      right_pointer -= 1;
    } else {
      let temp = nums[left_pointer];
      nums[left_pointer] = nums[right_pointer];
      nums[right_pointer] = temp;
      left_pointer += 1;
      right_pointer -= 1;
    }
  }
}

fn main() {
  let mut nums = vec![3,1,2,4,7,6];
  sort_array_by_parity(&mut nums);
  println!("{:?}", nums);
}