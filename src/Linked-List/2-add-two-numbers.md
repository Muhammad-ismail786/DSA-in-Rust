# 2. Add Two Numbers

**LeetCode:** https://leetcode.com/problems/add-two-numbers/

**Difficulty:** Medium

**Topic:** Linked List

**Language:** Rust

## Intuition

We are given two numbers represented by two linked lists.

Each node contains one digit, and the digits are stored in reverse order.

We need to add both numbers and return the result as a linked list.

Example:

```text
l1 = 2 -> 4 -> 3
l2 = 5 -> 6 -> 4
```

These linked lists represent:

```text
342 + 465 = 807
```

Our answer will be:

```text
7 -> 0 -> 8
```

We use a `carry` variable to store the extra value when the sum is greater than or equal to 10.

## Approach

1. Create a dummy node to start the result linked list.
2. Create a current pointer to track the last node.
3. Initialize carry to 0.
4. Continue the loop while either list has nodes or carry is not zero.
5. Start sum with the carry value.
6. If l1 has a node, add its value and move to the next node.
7. If l2 has a node, add its value and move to the next node.
8. Calculate carry.
9. Calculate the digit using `sum % 10`.
10. Create a new node containing that digit.
11. Attach the new node to the result list.
12. Return `dummy.next`.

## Example Dry Run

Input:

```text
l1 = 2 -> 4 -> 3
l2 = 5 -> 6 -> 4
```

| Round | l1 | l2 | Carry | Sum | Digit |
|---|---:|---:|---:|---:|---:|
| 1 | 2 | 5 | 0 | 7 | 7 |
| 2 | 4 | 6 | 0 | 10 | 0 |
| 3 | 3 | 4 | 1 | 8 | 8 |

Output:

```text
7 -> 0 -> 8
```

## Complexity

**Time Complexity:** O(max(n, m))

We visit each node of both linked lists once.

**Space Complexity:** O(max(n, m))

We create a new linked list to store the result.

## Code

```rust
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn add_two_numbers(
        l1: Option<Rc<RefCell<ListNode>>>,
        l2: Option<Rc<RefCell<ListNode>>>,
    ) -> Option<Rc<RefCell<ListNode>>> {
        let dummy = Rc::new(RefCell::new(ListNode::new(0)));
        let mut current = Rc::clone(&dummy);

        let mut l1 = l1;
        let mut l2 = l2;
        let mut carry = 0;

        while l1.is_some() || l2.is_some() || carry != 0 {
            let mut sum = carry;

            if let Some(node) = l1 {
                let borrowed = node.borrow();
                sum += borrowed.val;
                l1 = borrowed.next.clone();
            }

            if let Some(node) = l2 {
                let borrowed = node.borrow();
                sum += borrowed.val;
                l2 = borrowed.next.clone();
            }

            if sum >= 10 {
                carry = 1;
            } else {
                carry = 0;
            }

            let digit = sum % 10;
            let new_node = Rc::new(RefCell::new(ListNode::new(digit)));

            current.borrow_mut().next = Some(Rc::clone(&new_node));
            current = new_node;
        }

        dummy.borrow().next.clone()
    }
}
```

**Note:** LeetCode provides the `ListNode` definition and `Solution` structure. For LeetCode submission, use only the required function implementation.
