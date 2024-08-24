fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
	if nums.is_empty() {
		return 0;
	}
	let mut k: i32 = 1; // k will hold the count of unique elements
	// start from the second element and check for duplicates
	for i in 1..nums.len() {
		if nums[i] != nums[i - 1] {
			nums[k as usize] = nums[i]; // place the unique element at index k
			k += 1;
		}
	}
	k// return the number of unique elements
}

fn main() {
  let mut nums = vec![1, 1, 3, 5, 5, 6, 7, 7, 7, 8];
  let k = remove_duplicates(&mut nums);
  println!("{}", k);
}