class Solution {
    public int[] twoSum(int[] nums, int target) {
        HashMap<Integer, Integer> x = new HashMap(nums.length);

        for (int i = 0; i < nums.length; ++i) {
            if (x.containsKey(target - nums[i])) {
                return new int[] {i, x.get(target - nums[i])};
            } else {
                x.put(nums[i], i);
            }
        }

        return new int[] {};
    }
}
