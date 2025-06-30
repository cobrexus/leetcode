// https://leetcode.com/problems/longest-substring-without-repeating-characters/

class Solution {
public:
  int lengthOfLongestSubstring(string s) {
    if (s.size() < 1) {
      return s.size();
    }

    int start{};
    int longest{};

    unordered_map<char, size_t> pos{};

    for (int end{}; end < s.size(); ++end) {
      if (pos.contains(s[end]) && pos[s[end]] >= start) {
        start = pos[s[end]] + 1;
      }

      pos[s[end]] = end;
      longest = max(longest, end - start + 1);
    }

    return longest;
  }
};
