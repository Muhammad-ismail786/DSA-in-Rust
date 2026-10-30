# 206. Reverse Linked List

## Problem

Given the head of a singly linked list, reverse the list and return the new head.

## Intuition

We need to reverse the direction of every `next` pointer.

We use three variables:

- `prev`: Keeps the part of the list that has already been reversed.
- `current`: Points to the node we are currently processing.
- `next_node`: Temporarily saves the next node so we do not lose the remaining list.

Initially, `prev` is `None` because the reversed list is empty.

## Approach

Repeat these steps while `current` contains a node:

1. Save the next node in `next_node` using `take()`.
2. Point the current node's `next` to `prev`.
3. Move `prev` forward to the current node.
4. Move `current` forward to `next_node`.

When the loop finishes, `prev` points to the new head of the reversed list.

## Example

Original List:

`1 -> 2 -> 3 -> None`

Reversed List:

`3 -> 2 -> 1 -> None`

## Rust Code

```rust
impl Solution {
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut prev = None;
        let mut current = head;

        while let Some(mut node) = current {
            let next_node = node.next.take();

            node.next = prev;

            prev = Some(node);
            current = next_node;
        }

        prev
    }
}
