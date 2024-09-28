// https://leetcode.com/problems/median-of-two-sorted-arrays/

fn main() {}

struct Solution;

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let mut temp_1 = nums1.clone();
        let mut temp_2 = nums2.clone();
        temp_1.append(&mut temp_2);
        temp_1.sort_unstable();
        let len = temp_1.len();

        if len % 2 == 0 {
            let a = temp_1[len / 2 - 1] as f64;
            let b = temp_1[len / 2] as f64;
            return (a + b) / 2.0;
        }

        temp_1[len / 2] as f64
    }
}
