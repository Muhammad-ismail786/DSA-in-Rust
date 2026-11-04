# 228. Summary Ranges – Rust Solution

## Intuition

The main idea is to group consecutive numbers into ranges.

If numbers are consecutive, we keep them in the same range. If there is a gap, we finish the current range and start a new one.

For example:

Input:
```rust
[2, 3, 4, 7, 9, 10]
```

Output:
```text
["2->4", "7", "9->10"]
```

## Approach

1. Create an empty vector `result` to store the final ranges.
2. If the input vector is empty, return the empty result.
3. Use `start = 0` to remember the starting index of the current range.
4. Start the loop from index `1` and compare the current number with the previous number.
5. If the current number is not consecutive with the previous number:
   - If the range contains only one number, convert it to a string and add it to `result`.
   - Otherwise, create a range in `"start->end"` format using `format!()`.
6. Update `start = i` to begin a new range.
7. After the loop, add the last range.
8. Return the result vector.

## Complexity

- **Time Complexity:** O(n) – We traverse the input vector once.
- **Space Complexity:** O(n) – The result vector stores the ranges. Auxiliary space is O(1), excluding the output.

## Code

```rust
struct Solution;

impl Solution {
    fn summary_ranges(nums: Vec<i32>) -> Vec<String> {
        let mut result: Vec<String> = Vec::new();

        if nums.is_empty() {
            return result;
        }

        let mut start = 0;

        for i in 1..nums.len() {
            if nums[i] != nums[i - 1] + 1 {
                if start == i - 1 {
                    result.push(nums[start].to_string());
                } else {
                    result.push(format!("{}->{}", nums[start], nums[i - 1]));
                }

                start = i;
            }
        }

        if start == nums.len() - 1 {
            result.push(nums[start].to_string());
        } else {
            result.push(format!("{}->{}", nums[start], nums[nums.len() - 1]));
        }

        result
    }
}

fn main() {
    let nums = vec![2, 3, 4, 7, 9, 10];

    let result = Solution::summary_ranges(nums.clone());

    println!("Input: {:?}", nums);
    println!("Output: {:?}", result);
}
```
