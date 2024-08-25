fn move_zeroes(nums: &mut Vec<i32>) {
  let mut zero_pointer: i32 = 0;
  for read_pointer in 0..nums.len() {
    if nums[read_pointer] != 0 {
      let temp: i32 = nums[zero_pointer as usize];
      nums[zero_pointer as usize] = nums[read_pointer as usize];
      nums[read_pointer as usize] = temp;
      zero_pointer += 1;
    }
  }
}

fn main() {
  let mut nums = vec![0,1,0,3,12];
  move_zeroes(&mut nums);
}