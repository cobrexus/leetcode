// https://leetcode.com/problems/add-two-numbers/

class Solution {
    public ListNode addTwoNumbers(ListNode l1, ListNode l2) {
        ListNode newList = new ListNode();
        ListNode tail = newList;

        int carry = 0;

        while (l1 != null || l2 != null || carry != 0) {
            int val1 = l1 != null ? l1.val : 0;
            int val2 = l2 != null ? l2.val : 0;

            int sum = val1 + val2 + carry;
            int newVal = sum % 10;
            int newCarry = sum / 10;

            int currentDigit = newVal;
            carry = newCarry;

            ListNode temp = new ListNode(currentDigit);
            tail.next = temp;
            tail = tail.next;

            if (l1 != null) {
                l1 = l1.next;
            }

            if (l2 != null) {
                l2 = l2.next;
            }
        }

        return newList.next;
    }
}
