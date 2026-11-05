# 231. Power of Two

## Intuition

A power of two is a number that can be divided by 2 repeatedly until it becomes 1.

Example:

16 → 8 → 4 → 2 → 1

If the final number becomes 1, it is a power of two.

## Approach

1. First, check if n is less than or equal to 0. If yes, return false.
2. Create a mutable variable n because its value needs to change.
3. Use a while loop while n is completely divisible by 2.
4. Divide n by 2 in each iteration.
5. Finally, check whether n is equal to 1.
6. Return true if n == 1; otherwise, return false.

## Complexity

- **Time Complexity:** O(log n) — We divide the number by 2 repeatedly.
- **Space Complexity:** O(1) — We use constant extra space.

## Rust Code

```rust
struct Solution;

impl Solution {
    fn is_power_of_two(n: i32) -> bool {
        if n <= 0 {
            return false;
        }

        let mut n = n;

        while n % 2 == 0 {
            n = n / 2;
        }

        n == 1
    }
}

fn main() {
    let n: i32 = 16;

    let result = Solution::is_power_of_two(n);

    println!("Input: {}", n);
    println!("Is Power Of Two: {}", result);
}
```

## Example

Input: n = 16

Output: true

Explanation:

16 / 2 = 8

8 / 2 = 4

4 / 2 = 2

2 / 2 = 1

The final number is 1, so 16 is a power of two.
