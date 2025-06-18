// https://leetcode.com/problems/two-sum/

class Solution {
  public:
    vector<int> twoSum(vector<int>& nums, int target) {
        unordered_map<int, int> x;

        x.reserve(nums.size());

        for (int i{}; i < nums.size(); ++i) {
            if (x.contains(target - nums[i])) {
                return {i, x[target - nums[i]]};
            } else {
                x.insert({nums[i], i});
            }
        }

        return {};
    }
};
