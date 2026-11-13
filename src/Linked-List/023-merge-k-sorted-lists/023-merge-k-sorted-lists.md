
# 23. Merge k Sorted Lists

**Difficulty:** Hard
**Topic:** Linked List, Heap, Priority Queue
**Language:** Rust

## Problem Description

You are given an array of `k` linked lists. Each linked list is sorted in ascending order.

Merge all the linked lists into one sorted linked list and return it.

## Example

**Input:**

```text
lists = [[1,4,5],[1,3,4],[2,6]]
```

**Output:**

```text
[1,1,2,3,4,4,5,6]
```

**Explanation:**

All three linked lists are merged into one sorted linked list.

## Intuition

We need to merge multiple sorted linked lists into one sorted linked list.

Instead of checking every node manually, we can use a Min Heap.

A Min Heap always gives us the smallest available value.

We initially insert the first node of every non-empty linked list into the heap.

Then:

1. Remove the smallest node from the heap.
2. Add that node to the result linked list.
3. Move to the next node of the same input list.
4. If another node exists, insert it into the heap.
5. Repeat until the heap becomes empty.

## Approach

1. Create a `BinaryHeap` and use `Reverse` to make it behave like a Min Heap.
2. Insert the first node of every non-empty list into the heap.
3. Create a dummy node to build the result linked list.
4. Remove the smallest node from the heap.
5. Attach that node to the result list.
6. Move to the next node in the same input list.
7. Insert the next node into the heap if it exists.
8. Continue until the heap is empty.
9. Return `dummy.next`.

## Complexity

- **Time Complexity:** `O(N log k)`

  `N` is the total number of nodes and `k` is the number of linked lists. Each node is inserted into and removed from the heap at most once.

- **Space Complexity:** `O(k)`

  The heap stores at most one node candidate from each linked list.

## Code

```rust
use std::cmp::Reverse;
use std::collections::BinaryHeap;

impl Solution {
    pub fn merge_k_lists(
        mut lists: Vec<Option<Box<ListNode>>>,
    ) -> Option<Box<ListNode>> {
        let mut min_heap = BinaryHeap::new();

        for (index, list) in lists.iter().enumerate() {
            if let Some(node) = list {
                min_heap.push(Reverse((node.val, index)));
            }
        }

        let mut dummy = Box::new(ListNode::new(0));
        let mut tail = &mut dummy;

        while let Some(Reverse((_value, index))) = min_heap.pop() {
            let mut node = lists[index].take().unwrap();

            lists[index] = node.next.take();

            tail.next = Some(node);
            tail = tail.next.as_mut().unwrap();

            if let Some(next_node) = &lists[index] {
                min_heap.push(Reverse((next_node.val, index)));
            }
        }

        dummy.next
    }
}
```

## Key Concepts

### 1. BinaryHeap

Rust's `BinaryHeap` is a max heap by default. It returns the largest element first.

### 2. Reverse

`Reverse` changes the ordering, so the smallest element comes out first. This lets us use `BinaryHeap` as a Min Heap.

### 3. Option and Box

- `Option<Box<ListNode>>` represents a linked list node that may or may not exist.
- `Box<ListNode>` stores a node on the heap.
- `.take()` moves a value out of an `Option`, leaving `None` behind.
- `.next.take()` moves the next node out of the current node.

### 4. Dummy Node

A dummy node makes it easier to build the result linked list without special handling for the first node.
