# 226. Invert Binary Tree

## Intuition

The problem asks us to invert a binary tree by swapping the left and right children of every node.

We can use recursion to visit every node and swap its left and right children.

## Approach

1. Check if the root is `None`. If it is, return `None`.
2. Borrow the current node mutably to modify its children.
3. Swap the left and right children using `std::mem::swap()`.
4. Clone the child references to use them in recursive calls.
5. Recursively invert the left subtree.
6. Recursively invert the right subtree.
7. Finally, return the original root.

## Complexity

- **Time Complexity:** \(O(n)\) — We visit every node exactly once.
- **Space Complexity:** \(O(h)\) — Recursion uses stack space proportional to the height of the tree. In the worst case, it can be \(O(n)\).

## Code

See [226-invert-binary-tree.rs](226-invert-binary-tree.rs).

## Output

```text
Original Tree:
4 2 1 3 7 6 9

Inverted Tree:
4 7 9 6 2 3 1
```
