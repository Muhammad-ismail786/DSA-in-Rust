# 219. Contains Duplicate II

**Difficulty:** Easy
**Topics:** Array, Hash Table
**Language:** Rust

## Intuition

We need to find whether the array contains two equal numbers whose index difference is less than or equal to `k`.

We can use a `HashMap` to store each number and its most recent index.

## Approach

1. Create a `HashMap` to store numbers as keys and their indexes as values.
2. Traverse the array using `iter().enumerate()`.
3. Check whether the current number already exists in the HashMap.
4. If it exists, retrieve its previous index using `get()`.
5. Calculate the absolute difference between the current index and the previous index.
6. If the difference is less than or equal to `k`, return `true`.
7. Insert the current number and its index into the HashMap.
8. If no valid pair is found, return `false`.

## Complexity

- **Time Complexity:** O(n) — We traverse the array once. HashMap operations take O(1) average time.
- **Space Complexity:** O(n) — The HashMap can store up to n different numbers.

## Code

```rust
use std::collections::HashMap;

impl Solution {
    pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
        let mut mp: HashMap<i32, usize> = HashMap::new();

        for (i, &n) in nums.iter().enumerate() {
            if let Some(&previous_index) = mp.get(&n) {
                let distance = i.abs_diff(previous_index);

                if distance <= k as usize {
                    return true;
                }
            }

            mp.insert(n, i);
        }

        false
    }
}
```

## Important Rust Concepts

- `HashMap<i32, usize>`: Stores an integer number as the key and its array index as the value.
- `HashMap::new()`: Creates a new empty HashMap.
- `mut`: Allows the map to be modified.
- `iter()`: Iterates over array elements by reference.
- `enumerate()`: Provides both index and element.
- `mp.get(&n)`: Retrieves the value associated with key `n`.
- `Some`: Represents a value that exists.
- `None`: Represents a missing value.
- `mp.insert(n, i)`: Inserts or updates a key-value pair.
- `abs_diff()`: Calculates the absolute difference between two indexes.
- `usize`: Rust's commonly used type for array indexes.
