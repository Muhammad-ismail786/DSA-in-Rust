# 234. Palindrome Linked List

## Intuition

We need to check whether a linked list reads the same forward and backward.

Example:

`1 -> 2 -> 2 -> 1`

This is a palindrome because the values are the same from both directions.

## Approach

1. Use two pointers, `slow` and `fast`, to find the middle of the linked list.
2. Reverse the second half of the linked list.
3. Compare the first half with the reversed second half.
4. If all corresponding values match, return `true`.
5. Otherwise, return `false`.

## Complexity

- **Time Complexity:** O(n) — We traverse the linked list to find the middle, reverse the second half, and compare values.
- **Space Complexity:** O(1) — We use only a few pointers and no extra data structure.

## Code

```rust
impl Solution {
    pub fn is_palindrome(head: Option<Rc<RefCell<ListNode>>>) -> bool {
        let mut slow = head.clone();
        let mut fast = head.clone();

        while let Some(fast_node) = fast.clone() {
            let next = fast_node.borrow().next.clone();

            if next.is_none() {
                break;
            }

            slow = slow.and_then(|node| node.borrow().next.clone());

            fast = next.and_then(|node| node.borrow().next.clone());
        }

        let mut prev = None;
        let mut curr = slow;

        while let Some(node) = curr {
            let next = node.borrow().next.clone();

            node.borrow_mut().next = prev.clone();

            prev = Some(node);
            curr = next;
        }

        let mut first = head;
        let mut second = prev;

        while let Some(second_node) = second {
            let first_node = match first {
                Some(node) => node,
                None => return false,
            };

            if first_node.borrow().val != second_node.borrow().val {
                return false;
            }

            first = first_node.borrow().next.clone();
            second = second_node.borrow().next.clone();
        }

        true
    }
}
```
