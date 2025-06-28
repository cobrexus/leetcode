// https://leetcode.com/problems/longest-substring-without-repeating-characters/

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        if s.len() < 1 {
            return s.len() as i32;
        }

        let mut start = 0;
        let mut longest = 0;

        let mut pos = std::collections::HashMap::<char, usize>::new();

        for (end_idx, end) in s.chars().enumerate() {
            if let Some(prev) = pos.get_mut(&end) {
                if *prev >= start {
                    start = *prev + 1;
                }

                *prev = end_idx;
            } else {
                pos.insert(end, end_idx);
            }
            longest = std::cmp::max(longest, end_idx - start + 1);
        }

        longest as i32
    }
}
