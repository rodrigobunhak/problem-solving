fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
	if nums.is_empty() {
		return 0;
	}
	let mut k: i32 = 1;
	for i in 1..nums.len() {
		if nums[i] != nums[i - 1] {
			nums[k as usize] = nums[i];
			k += 1;
		}
	}
	k
}

fn main() {
  let mut nums = vec![1, 1, 3, 5, 5, 6, 7, 7, 7, 8];
  let k = remove_duplicates(&mut nums);
  println!("{}", k);
}