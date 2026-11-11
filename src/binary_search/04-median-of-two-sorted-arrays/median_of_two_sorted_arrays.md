
# 4. Median of Two Sorted Arrays

**Difficulty:** Hard

**Topics:** Array, Binary Search, Divide and Conquer

## Intuition

We are given two sorted arrays.

We need to find the median without merging both arrays.

The main idea is to divide both arrays into two parts:

- Left half
- Right half

The left half should contain half of the total elements.

All elements in the left half must be smaller than or equal to all elements in the right half.

We use Binary Search on the smaller array to find the correct partition.

## Approach

1. First, make sure nums1 is the smaller array.
2. Calculate the sizes of both arrays.
3. Initialize low = 0 and high = m.
4. Use Binary Search to calculate cut1.
5. Calculate cut2 so that the left partition contains half of the total elements.
6. Find four boundary values:
   - l1: Last element on the left side of nums1.
   - r1: First element on the right side of nums1.
   - l2: Last element on the left side of nums2.
   - r2: First element on the right side of nums2.
7. Check whether the partition is correct:
   l1 <= r2 and l2 <= r1
8. If the total number of elements is even, return the average of the two middle elements.
9. If the total number of elements is odd, return the maximum element from the left partition.
10. If the partition is incorrect, adjust the Binary Search boundaries.

## Complexity

- Time complexity: O(log(min(m, n)))
- Space complexity: O(1)

## Code

```rust
impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {

        if nums1.len() > nums2.len() {
            return Self::find_median_sorted_arrays(nums2, nums1);
        }

        let m = nums1.len();
        let n = nums2.len();

        let mut low = 0;
        let mut high = m;

        while low <= high {
            let cut1 = low + (high - low) / 2;
            let cut2 = (m + n + 1) / 2 - cut1;

            let l1 = if cut1 == 0 {
                i32::MIN
            } else {
                nums1[cut1 - 1]
            };

            let r1 = if cut1 == m {
                i32::MAX
            } else {
                nums1[cut1]
            };

            let l2 = if cut2 == 0 {
                i32::MIN
            } else {
                nums2[cut2 - 1]
            };

            let r2 = if cut2 == n {
                i32::MAX
            } else {
                nums2[cut2]
            };

            if l1 <= r2 && l2 <= r1 {
                if (m + n) % 2 == 0 {
                    let left_max = l1.max(l2);
                    let right_min = r1.min(r2);

                    return (left_max as f64 + right_min as f64) / 2.0;
                }

                return l1.max(l2) as f64;
            } else if l1 > r2 {
                high = cut1 - 1;
            } else {
                low = cut1 + 1;
            }
        }

        0.0
    }
}
```
