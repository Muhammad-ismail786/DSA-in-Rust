# 42. Trapping Rain Water

**Difficulty:** Hard

**Topics:** Array, Two Pointers, Dynamic Programming, Stack

## Intuition

The problem asks us to calculate how much rainwater can be trapped between the bars.

Water can be trapped when there are taller bars on both the left and right sides of a position.

For each position, we consider:

- `left_max`: Maximum height found from the left side.
- `right_max`: Maximum height found from the right side.
- `height[i]`: Current bar height.

We use the Two Pointers approach to calculate trapped rainwater efficiently without using extra arrays.

## Approach

1. Initialize two pointers:
   - `left = 0` points to the first element.
   - `right = height.len() - 1` points to the last element.

2. Initialize `left_max` and `right_max` to store the maximum heights found from both sides.

3. Initialize `water = 0` to store the total trapped rainwater.

4. Run a while loop while `left < right`.

5. Compare the heights at both pointers:
   - If `height[left] < height[right]`, process the left side.
   - Otherwise, process the right side.

6. For the left side:
   - If the current height is greater than or equal to `left_max`, update `left_max`.
   - Otherwise, add `left_max - height[left]` to `water`.
   - Move `left` forward.

7. For the right side:
   - If the current height is greater than or equal to `right_max`, update `right_max`.
   - Otherwise, add `right_max - height[right]` to `water`.
   - Move `right` backward.

8. Return the total trapped rainwater.

## Complexity

- **Time Complexity:** O(n)
  - Each pointer moves through the array at most once.

- **Space Complexity:** O(1)
  - Only a constant number of extra variables are used.

## Rust Code

```rust
fn trap(height: Vec<i32>) -> i32 {

    let mut left: usize = 0;
    let mut right: usize = height.len() - 1;

    let mut left_max = 0;
    let mut right_max = 0;

    let mut water = 0;

    while left < right {

        if height[left] < height[right] {

            if height[left] >= left_max {
                left_max = height[left];
            } else {
                water += left_max - height[left];
            }

            left += 1;

        } else {

            if height[right] >= right_max {
                right_max = height[right];
            } else {
                water += right_max - height[right];
            }

            right -= 1;
        }
    }

    water
}

fn main() {

    let height = vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1];

    let result = trap(height.clone());

    println!("Input: {:?}", height);
    println!("Trapped Rain Water: {}", result);
}
```

## LeetCode Rust Code

```rust
impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {

        let mut left: usize = 0;
        let mut right: usize = height.len() - 1;

        let mut left_max = 0;
        let mut right_max = 0;

        let mut water = 0;

        while left < right {

            if height[left] < height[right] {

                if height[left] >= left_max {
                    left_max = height[left];
                } else {
                    water += left_max - height[left];
                }

                left += 1;

            } else {

                if height[right] >= right_max {
                    right_max = height[right];
                } else {
                    water += right_max - height[right];
                }

                right -= 1;
            }
        }

        water
    }
}
```

## Test Cases

**Example 1:**

Input:
```text
[0,1,0,2,1,0,1,3,2,1,2,1]
```

Output:
```text
6
```

**Example 2:**

Input:
```text
[4,2,0,3,2,5]
```

Output:
```text
9
```

**Example 3:**

Input:
```text
[3,0,2]
```

Output:
```text
2
```
