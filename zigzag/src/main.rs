// https://leetcode.com/problems/zigzag-conversion/

fn main() {}

struct Solution;

impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        if num_rows == 1 {
            return s;
        }

        let s = s.as_bytes();
        let num_rows = num_rows as usize;

        let mut grid: Vec<Vec<u8>> = vec![vec![0; s.len()]; num_rows];
        let mut col = 0;
        let mut row = 0;
        let mut rev = false;

        for c in s {
            grid[row][col] = *c;

            if !rev {
                row += 1;
            }

            if rev {
                row -= 1;
                col += 1;
            }

            if row == num_rows {
                rev = true;
                row -= 2;
                col += 1;
            }

            if row == 0 {
                rev = false;
            }
        }

        grid.iter()
            .flatten()
            .filter(|&&x| x != 0)
            .map(|&x| x as char)
            .collect()
    }
}
