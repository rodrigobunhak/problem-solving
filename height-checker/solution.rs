fn height_checker(heights: Vec<i32>) -> i32 {
  let mut expected = heights.clone();
  expected.sort();
  let mut count = 0;
  for i in 0..heights.len() {
    if heights[i] != expected[i] {
      count += 1;
    }
  }
  count
}

fn main() {
  let heights = vec![1, 1, 4, 2, 1, 3];
  println!("{}", height_checker(heights));
}