function moveZeroes(nums: number[]): void {
  let zeroPointer = 0;
  for (let readPointer = 0; readPointer < nums.length; readPointer++) {
    if (nums[readPointer] !== 0) {
      [nums[zeroPointer], nums[readPointer]] = [nums[readPointer], nums[zeroPointer]];
      zeroPointer++;
    }
  }
};

moveZeroes([0,1,0,3,12]);