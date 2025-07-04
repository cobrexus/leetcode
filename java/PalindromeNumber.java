// https://leetcode.com/problems/palindrome-number/

class Solution {
    public boolean isPalindrome(int x) {
        int xCopy = x;
        int temp = 0;

        while (x > 0) {
            temp *= 10;
            temp += x % 10;
            x /= 10;
        }

        return temp == xCopy;
    }
}
