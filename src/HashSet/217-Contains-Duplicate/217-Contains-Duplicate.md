# 217. Contains Duplicate

## Intuition

We need to check whether the given array contains any duplicate numbers.

We can use a `HashSet` to store numbers we have already visited. If a number is already present in the HashSet, it means a duplicate exists, so we return `true`.

## Approach

1. Import `HashSet` from Rust's standard library.
2. Create an empty mutable HashSet named `seen` using `HashSet::new()`.
3. Iterate through each number in the input vector using a for loop.
4. Check whether the current number already exists in the HashSet using `seen.contains(&n)`.
5. If the number exists, return `true` because a duplicate is found.
6. Otherwise, insert the number into the HashSet using `seen.insert(n)`.
7. If the loop finishes without finding any duplicate, return `false`.

## Complexity

- **Time Complexity:** O(n) average — We traverse the vector once. HashSet lookup and insertion take O(1) average time.
- **Space Complexity:** O(n) — In the worst case, the HashSet stores all unique numbers.

## Code

```rust
use std::collections::HashSet;

impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut seen = HashSet::new();

        for n in nums {
            if seen.contains(&n) {
                return true;
            }

            seen.insert(n);
        }

        false
    }
}
```
