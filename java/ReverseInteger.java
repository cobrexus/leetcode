// https://leetcode.com/problems/reverse-integer/

class Solution {
    public int reverse(int x) {
        if (x == 0) {
            return 0;
        }

        int xCopy = x;
        int temp = 0;

        if (x < 0) {
            x = -x;

            while (x > 0 && temp <= Math.abs(Integer.MIN_VALUE / 10)) {
                temp *= 10;
                temp += x % 10;
                x /= 10;
            }
        } else {
            while (x > 0 && temp <= Integer.MAX_VALUE / 10) {
                temp *= 10;
                temp += x % 10;
                x /= 10;
            }
        }

        if (x != 0) {
            return 0;
        }

        if (xCopy < 0) {
            return -temp;
        }

        return temp;
    }
}
