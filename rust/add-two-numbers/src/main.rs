// https://leetcode.com/problems/add-two-numbers/

impl Solution {
    pub fn add_two_numbers(
        mut l1: Option<Box<ListNode>>,
        mut l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        fn add(n1: i32, n2: i32, carry: i32) -> (i32, i32) {
            let sum = n1 + n2 + carry;
            let new_val = sum % 10;
            let new_carry = sum / 10;

            (new_val, new_carry)
        }

        let mut head = Box::new(ListNode { val: 0, next: None });
        let mut tail = &mut head.next;

        let mut carry = 0;
        let mut val1;
        let mut val2;
        let mut new_val;

        loop {
            val1 = match l1 {
                Some(ref node) => node.val,
                None => 0,
            };

            val2 = match l2 {
                Some(ref node) => node.val,
                None => 0,
            };

            (new_val, carry) = add(val1, val2, carry);

            tail = &mut tail
                .insert(Box::new(ListNode {
                    val: new_val,
                    next: None,
                }))
                .next;

            if let Some(node) = l1 {
                l1 = node.next;
            }

            if let Some(node) = l2 {
                l2 = node.next;
            }

            if l1.is_none() && l2.is_none() && carry == 0 {
                break;
            }
        }

        head.next
    }
}
