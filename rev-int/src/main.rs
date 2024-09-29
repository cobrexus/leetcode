// https://leetcode.com/problems/reverse-integer/

fn main() {}

struct Solution;

impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let z = x;
        let mut x = x.abs();
        let mut y = 0;

        while x > 0 {
            if y > i32::MAX / 10 {
                return 0;
            }

            y *= 10;
            y += x % 10;
            x /= 10;
        }

        if z.is_negative() {
            y * -1
        } else {
            y
        }
    }
}
