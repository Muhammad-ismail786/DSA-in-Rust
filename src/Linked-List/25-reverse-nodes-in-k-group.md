# LeetCode 25 — Reverse Nodes in k-Group (Rust)

## Intuition

We need to reverse the linked list in groups of `k` nodes.

If fewer than `k` nodes remain, those nodes must stay unchanged.

We reverse the links between nodes, not their values.

## Approach

1. Check whether at least `k` nodes are available.
2. If fewer than `k` nodes exist, return the original head.
3. Use `prev` and `curr` to reverse the first `k` nodes.
4. Find the tail of the reversed group.
5. Connect that tail to the recursively reversed remaining list.
6. Return the new head of the reversed group.

### Important Rust concepts

- `Option<Box<ListNode>>`: Represents a node that may or may not exist.
- `Box<ListNode>`: Stores a linked-list node on the heap.
- `.as_ref()`: Borrows the value without taking ownership.
- `.take()`: Takes the value and replaces the original with `None`.
- `Some(node)`: Wraps a node inside an `Option`.

## Complexity

- **Time complexity:** O(n) — Each node is visited a constant number of times.
- **Space complexity:** O(n/k) — Recursive call stack, worst case O(n) when k = 1.

## Code — Rust

```rust
impl Solution {
    pub fn reverse_k_group(
        head: Option<Box<ListNode>>,
        k: i32
    ) -> Option<Box<ListNode>> {

        if k <= 1 {
            return head;
        }

        let mut check = head.as_ref();

        for _ in 0..k {
            match check {
                Some(node) => {
                    check = node.next.as_ref();
                }
                None => {
                    return head;
                }
            }
        }

        let mut prev: Option<Box<ListNode>> = None;
        let mut curr = head;

        for _ in 0..k {
            let mut node = curr.unwrap();

            curr = node.next.take();
            node.next = prev;
            prev = Some(node);
        }

        let mut reversed_head = prev;
        let mut tail = reversed_head.as_mut().unwrap();

        while tail.next.is_some() {
            tail = tail.next.as_mut().unwrap();
        }

        tail.next = Self::reverse_k_group(curr, k);

        reversed_head
    }
}
```

## Example

**Input:**
```text
head = [1,2,3,4,5]
k = 2
```

**Output:**
```text
[2,1,4,3,5]
```

**Explanation:**

- `[1,2]` becomes `[2,1]`.
- `[3,4]` becomes `[4,3]`.
- `[5]` remains unchanged because fewer than 2 nodes remain.
