function sortArrayByParity(nums: number[]): number[] {
  let leftPointer = 0;
  let rightPointer = nums.length - 1;
  while (leftPointer < rightPointer) {
    if (nums[leftPointer] % 2 === 0) {
      leftPointer++;
    }
    else if (nums[rightPointer] % 2 !== 0) {
      rightPointer--;
    }
    else {
      const temp = nums[rightPointer];
      nums[rightPointer] = nums[leftPointer];
      nums[leftPointer] = temp;
      leftPointer++;
      rightPointer--;
    }
  }
  return nums;
};

console.log(sortArrayByParity([3,1,2,4,7,6]));