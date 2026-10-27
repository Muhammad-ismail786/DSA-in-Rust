# 202. Happy Number

**Language:** Rust

## Intuition

We repeatedly calculate the sum of the squares of each digit.

If the number becomes `1`, it is a Happy Number.

If a number repeats before reaching `1`, the process is stuck in a cycle, so the number is not happy.

## Approach

1. Import `HashSet` to store previously visited numbers.
2. Create a mutable variable `n` because its value changes.
3. Create an empty `HashSet` named `seen`.
4. Run a loop while `n != 1`.
5. If `n` already exists in `seen`, return `false`.
6. Insert `n` into `seen`.
7. Initialize `sum` to `0`.
8. Extract the last digit using `n % 10`.
9. Add the square of the digit to `sum`.
10. Remove the last digit using `n /= 10`.
11. Set `n = sum` and repeat.
12. If `n` reaches `1`, return `true`.

**Example:**

`7 → 49 → 97 → 130 → 10 → 1`

The number reaches `1`, so the result is `true`.

## Complexity

- **Time Complexity:** `O(log n)` per digit-squaring operation. Overall time depends on the number of distinct values visited.
- **Space Complexity:** `O(k)`, where `k` is the number of distinct values stored in `seen`.

## Code

```rust
use std::collections::HashSet;

impl Solution {
    pub fn is_happy(mut n: i32) -> bool {
        let mut seen = HashSet::new();

        while n != 1 {
            if seen.contains(&n) {
                return false;
            }

            seen.insert(n);

            let mut sum = 0;

            while n > 0 {
                let digit = n % 10;
                sum += digit * digit;
                n /= 10;
            }

            n = sum;
        }

        true
    }
}
```
