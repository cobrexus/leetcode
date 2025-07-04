// https://leetcode.com/problems/median-of-two-sorted-arrays/

class Solution {
    public double findMedianSortedArrays(int[] nums1, int[] nums2) {
        int[] temp = new int[nums1.length + nums2.length];
        int tempIdx = 0;
        int idx1 = 0;
        int idx2 = 0;

        while (idx1 < nums1.length && idx2 < nums2.length) {
            if (nums1[idx1] <= nums2[idx2]) {
                temp[tempIdx++] = nums1[idx1++];
            } else {
                temp[tempIdx++] = nums2[idx2++];
            }
        }

        while (idx1 < nums1.length) {
            temp[tempIdx++] = nums1[idx1++];
        }

        while (idx2 < nums2.length) {
            temp[tempIdx++] = nums2[idx2++];
        }

        if (temp.length % 2 == 0) {
            int a = temp[temp.length / 2 - 1];
            int b = temp[temp.length / 2];
            return (double)(a + b) / 2;
        } else {
            return temp[temp.length / 2];
        }

    }
}
