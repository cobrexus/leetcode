// https://leetcode.com/problems/string-to-integer-atoi/

impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        let s = s.as_bytes();
        let mut s_start: usize = 0;
        let s_end = s.len();

        let mut output = 0;
        let mut negative = false;

        if s_start == s_end {
            return output;
        }

        while s_start != s_end && s[s_start] == b' ' {
            s_start += 1;
        }

        if s_start == s_end {
            return output;
        }

        if s[s_start] == b'-' {
            negative = true;
            s_start += 1;
        } else if s[s_start] == b'+' {
            s_start += 1;
        }

        if s_start == s_end {
            return output;
        }

        while s_start != s_end && s[s_start] == b'0' {
            s_start += 1;
        }

        while s_start != s_end {
            if s[s_start] == b'0' {
                if negative && i32::MIN / 10 > output * -1 {
                    return i32::MIN;
                } else if !negative && i32::MAX / 10 < output {
                    return i32::MAX;
                } else {
                    output *= 10;
                }
            } else if s[s_start] == b'1' {
                if negative && (i32::MIN + 1) / 10 > output * -1 {
                    return i32::MIN;
                } else if !negative && (i32::MAX - 1) / 10 < output {
                    return i32::MAX;
                } else {
                    output *= 10;
                    output += 1;
                }
            } else if s[s_start] == b'2' {
                if negative && (i32::MIN + 2) / 10 > output * -1 {
                    return i32::MIN;
                } else if !negative && (i32::MAX - 2) / 10 < output {
                    return i32::MAX;
                } else {
                    output *= 10;
                    output += 2;
                }
            } else if s[s_start] == b'3' {
                if negative && (i32::MIN + 3) / 10 > output * -1 {
                    return i32::MIN;
                } else if !negative && (i32::MAX - 3) / 10 < output {
                    return i32::MAX;
                } else {
                    output *= 10;
                    output += 3;
                }
            } else if s[s_start] == b'4' {
                if negative && (i32::MIN + 4) / 10 > output * -1 {
                    return i32::MIN;
                } else if !negative && (i32::MAX - 4) / 10 < output {
                    return i32::MAX;
                } else {
                    output *= 10;
                    output += 4;
                }
            } else if s[s_start] == b'5' {
                if negative && (i32::MIN + 5) / 10 > output * -1 {
                    return i32::MIN;
                } else if !negative && (i32::MAX - 5) / 10 < output {
                    return i32::MAX;
                } else {
                    output *= 10;
                    output += 5;
                }
            } else if s[s_start] == b'6' {
                if negative && (i32::MIN + 6) / 10 > output * -1 {
                    return i32::MIN;
                } else if !negative && (i32::MAX - 6) / 10 < output {
                    return i32::MAX;
                } else {
                    output *= 10;
                    output += 6;
                }
            } else if s[s_start] == b'7' {
                if negative && (i32::MIN + 7) / 10 > output * -1 {
                    return i32::MIN;
                } else if !negative && (i32::MAX - 7) / 10 < output {
                    return i32::MAX;
                } else {
                    output *= 10;
                    output += 7;
                }
            } else if s[s_start] == b'8' {
                if negative && (i32::MIN + 8) / 10 > output * -1 {
                    return i32::MIN;
                } else if !negative && (i32::MAX - 8) / 10 < output {
                    return i32::MAX;
                } else {
                    output *= 10;
                    output += 8;
                }
            } else if s[s_start] == b'9' {
                if negative && (i32::MIN + 9) / 10 > output * -1 {
                    return i32::MIN;
                } else if !negative && (i32::MAX - 9) / 10 < output {
                    return i32::MAX;
                } else {
                    output *= 10;
                    output += 9;
                }
            } else {
                break;
            }

            s_start += 1;
        }

        if negative {
            output * -1
        } else {
            output
        }
    }
}
