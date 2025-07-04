// https://leetcode.com/problems/longest-substring-without-repeating-characters/

class Solution {
    public int lengthOfLongestSubstring(String s) {
        if (s.length() < 1) {
            return s.length();
        }

        int start = 0;
        int longest = 0;

        HashMap<Character, Integer> pos = new HashMap();

        for (int end = 0; end < s.length(); ++end) {
            char c = s.charAt(end);

            if (pos.containsKey(c) && pos.get(c) >= start) {
                start = pos.get(c) + 1;
            }

            pos.put(c, end);
            longest = Math.max(longest, end - start + 1);
        }

        return longest;
    }
}
