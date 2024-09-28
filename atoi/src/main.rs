// https://leetcode.com/problems/string-to-integer-atoi/

fn main() {
    println!("{}", Solution::my_atoi(" -1010023630o4".to_string()));
}

struct Solution;

impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        let mut s = s.trim().chars().peekable();

        let mut is_negative = false;
        let mut output = 0;

        if let Some('-') = s.peek() {
            is_negative = true;
            s.next();
        } else if let Some('+') = s.peek() {
            s.next();
        }

        for n in s {
            if ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'].contains(&n) {
                if output > i32::MAX / 10 && !is_negative {
                    output = i32::MAX;
                    break;
                }

                if output * -1 < i32::MIN / 10 && is_negative {
                    output = i32::MIN;
                    is_negative = false;
                    break;
                }

                output *= 10;

                match n {
                    '0' => (),
                    '1' => {
                        if output > i32::MAX - 1 && !is_negative {
                            output = i32::MAX;
                            break;
                        }

                        if output * -1 < i32::MIN + 1 && is_negative {
                            output = i32::MIN;
                            is_negative = false;
                            break;
                        }

                        output += 1;
                    }
                    '2' => {
                        if output > i32::MAX - 2 && !is_negative {
                            output = i32::MAX;
                            break;
                        }

                        if output * -1 < i32::MIN + 2 && is_negative {
                            output = i32::MIN;
                            is_negative = false;
                            break;
                        }

                        output += 2;
                    }
                    '3' => {
                        if output > i32::MAX - 3 && !is_negative {
                            output = i32::MAX;
                            break;
                        }

                        if output * -1 < i32::MIN + 3 && is_negative {
                            output = i32::MIN;
                            is_negative = false;
                            break;
                        }

                        output += 3;
                    }
                    '4' => {
                        if output > i32::MAX - 4 && !is_negative {
                            output = i32::MAX;
                            break;
                        }

                        if output * -1 < i32::MIN + 4 && is_negative {
                            output = i32::MIN;
                            is_negative = false;
                            break;
                        }

                        output += 4;
                    }
                    '5' => {
                        if output > i32::MAX - 5 && !is_negative {
                            output = i32::MAX;
                            break;
                        }

                        if output * -1 < i32::MIN + 5 && is_negative {
                            output = i32::MIN;
                            is_negative = false;
                            break;
                        }

                        output += 5;
                    }
                    '6' => {
                        if output > i32::MAX - 6 && !is_negative {
                            output = i32::MAX;
                            break;
                        }

                        if output * -1 < i32::MIN + 6 && is_negative {
                            output = i32::MIN;
                            is_negative = false;
                            break;
                        }

                        output += 6;
                    }
                    '7' => {
                        if output > i32::MAX - 7 && !is_negative {
                            output = i32::MAX;
                            break;
                        }

                        if output * -1 < i32::MIN + 7 && is_negative {
                            output = i32::MIN;
                            is_negative = false;
                            break;
                        }

                        output += 7;
                    }
                    '8' => {
                        if output > i32::MAX - 8 && !is_negative {
                            output = i32::MAX;
                            break;
                        }

                        if output * -1 < i32::MIN + 8 && is_negative {
                            output = i32::MIN;
                            is_negative = false;
                            break;
                        }

                        output += 8;
                    }
                    '9' => {
                        if output > i32::MAX - 9 && !is_negative {
                            output = i32::MAX;
                            break;
                        }

                        if output * -1 < i32::MIN + 9 && is_negative {
                            output = i32::MIN;
                            is_negative = false;
                            break;
                        }

                        output += 9;
                    }
                    _ => (),
                }
            } else {
                break;
            }
        }

        if is_negative {
            output * -1
        } else {
            output
        }
    }
}
