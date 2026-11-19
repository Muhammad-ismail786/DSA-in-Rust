# Intuition

We need to find the smallest missing positive integer from an unsorted array.

A positive integer means a number greater than 0.

For example:

```rust
let nums = vec![3, 4, -1, 1];
```

The smallest missing positive integer is `2`.

Instead of sorting the array, we can use Cyclic Sort to place each number at its correct index.

For example:

- Number 1 belongs at index 0.
- Number 2 belongs at index 1.
- Number 3 belongs at index 2.
- Number 4 belongs at index 3.

# Approach

1. Find the length of the vector using `nums.len()`.
2. Traverse the vector using a `for` loop.
3. If the current number is between 1 and n, move it to its correct index.
4. Use `nums.swap()` to exchange two elements.
5. Avoid unnecessary swaps by checking whether the correct position already contains the same number.
6. Traverse the vector again.
7. If `nums[i] != i + 1`, return `i + 1` because that positive number is missing.
8. If all positions contain their correct numbers, return `n + 1`.

# Complexity

- **Time Complexity:** O(n)

  Every number is moved to its correct position at most once.

- **Space Complexity:** O(1)

  We modify the vector in place without using an additional array.

# Code

```rust
struct Solution;

impl Solution {
    pub fn first_missing_positive(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        let n = nums.len();

        for i in 0..n {
            while nums[i] >= 1
                && nums[i] <= n as i32
                && nums[(nums[i] - 1) as usize] != nums[i]
            {
                let correct_index = (nums[i] - 1) as usize;
                nums.swap(i, correct_index);
            }
        }

        for i in 0..n {
            if nums[i] != (i as i32) + 1 {
                return (i as i32) + 1;
            }
        }

        (n as i32) + 1
    }
}

fn main() {
    let nums = vec![3, 4, -1, 1];

    let result = Solution::first_missing_positive(nums);

    println!("First Missing Positive: {}", result);
}
```

**Output:**

```text
First Missing Positive: 2
```
