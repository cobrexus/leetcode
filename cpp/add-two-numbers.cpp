// https://leetcode.com/problems/add-two-numbers/

class Solution {
  private:
    pair<int, int> add(int n1, int n2, int carry) {
        int sum{n1 + n2 + carry};
        int new_val{sum % 10};
        int new_carry{sum / 10};
        return {new_val, new_carry};
    }

    void add_node(int val, ListNode*& tail) {
        ListNode* temp{new ListNode{val}};
        tail->next = temp;
        tail = tail->next;
    }

  public:
    ListNode* addTwoNumbers(ListNode* l1, ListNode* l2) {
        ListNode* new_list{new ListNode{}};
        ListNode* tail{new_list};

        int carry{};
        int val1{};
        int val2{};
        int current_digit{};

        while (l1 || l2 || carry) {
            val1 = l1 ? l1->val : 0;
            val2 = l2 ? l2->val : 0;

            pair<int, int> p{add(val1, val2, carry)};
            current_digit = p.first;
            carry = p.second;

            add_node(current_digit, tail);

            if (l1) {
                l1 = l1->next;
            }

            if (l2) {
                l2 = l2->next;
            }
        }

        return new_list->next;
    }
};
