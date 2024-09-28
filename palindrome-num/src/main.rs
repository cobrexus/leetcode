// https://leetcode.com/problems/palindrome-number/

fn main() {
    println!("{}", Solution::is_palindrome(121));
}

struct Solution;

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x.is_negative() {
            return false;
        }

        let mut x = x;
        let y = x;
        let mut z = 0;

        while x > 0 {
            z *= 10;
            z += x % 10;
            x /= 10;
        }

        y == z
    }
}
