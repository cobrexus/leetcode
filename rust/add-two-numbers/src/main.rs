// https://leetcode.com/problems/add-two-numbers/

impl Solution {
    pub fn add_two_numbers(
        mut l1: Option<Box<ListNode>>,
        mut l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut head = Box::new(ListNode { val: 0, next: None });
        let mut tail = &mut head.next;

        let mut carry = 0;

        while (&l1, &l2, carry) != (&None, &None, 0) {
            let [val1, val2] = [&l1, &l2].map(|l| match l {
                Some(node) => node.val,
                None => 0,
            });

            let sum = val1 + val2 + carry;
            let new_val = sum % 10;
            carry = sum / 10;

            tail = &mut tail
                .insert(Box::new(ListNode {
                    val: new_val,
                    next: None,
                }))
                .next;

            for l in [&mut l1, &mut l2] {
                if let Some(node) = l.take() {
                    *l = node.next;
                }
            }
        }

        head.next
    }
}
