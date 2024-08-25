function removeDuplicates(nums: number[]): number {
  if (nums.length === 0) {
    return 0;
  };
  let k = 1; // k will hold the count of unique elements
  // start from the second element and check for duplicates
  for (let i = 1; i < nums.length; i++) {
    if (nums[i] !== nums[i - 1]) {
      nums[k] = nums[i]; // place the unique element at index k
      k++; 
    }
  }
  return k; // return the number of unique elements
};

console.log(removeDuplicates([1, 1, 3, 5, 5, 6, 7, 7, 7, 8]));