
# 203. Remove Linked List Elements

## Intuition
We need to remove all nodes from the linked list whose value equals `val`.

We traverse the linked list and update links to skip matching nodes.

## Approach
1. Create a mutable reference to the head.
2. Traverse the linked list while a node exists.
3. If the current node's value equals `val`, remove it and replace it with its next node.
4. Otherwise, move to the next node.
5. Return the updated head.

## Complexity

- **Time Complexity:** O(n), where n is the number of nodes.
- **Space Complexity:** O(1), because we use a constant amount of extra space.

## Code

```rust
impl Solution {
    pub fn remove_elements(
        mut head: Option<Box<ListNode>>,
        val: i32,
    ) -> Option<Box<ListNode>> {
        let mut current = &mut head;

        while current.is_some() {
            if current.as_ref().unwrap().val == val {
                *current = current.take().unwrap().next;
            } else {
                current = &mut current.as_mut().unwrap().next;
            }
        }

        head
    }
}
```
