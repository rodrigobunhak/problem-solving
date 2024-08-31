function heightChecker(heights: number[]): number {
  let expected = [...heights].sort();
  let count = 0;
  for (let i = 0; i < heights.length; i++) {
    if (heights[i] !== expected[i]) {
      count++;
    }
  }
  return count;
};

console.log(heightChecker([1,1,4,2,1,3]));