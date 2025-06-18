// https://leetcode.com/problems/container-with-most-water/

fn main() {}

struct Solution;

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut max_area = 0;
        let mut i = 0;
        let mut j = height.len() - 1;

        while i < j {
            let h = std::cmp::min(height[i], height[j]);
            let width = (j - i) as i32;
            max_area = std::cmp::max(max_area, h * width);

            if height[i] < height[j] {
                i += 1;
            } else {
                j -= 1;
            }
        }

        max_area
    }
}
