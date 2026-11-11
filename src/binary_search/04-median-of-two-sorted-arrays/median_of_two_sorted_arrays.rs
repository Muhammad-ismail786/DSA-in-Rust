
struct Solution;

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {

        // Always perform binary search on the smaller array
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

            // Find boundary values
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

            // Check if partition is correct
            if l1 <= r2 && l2 <= r1 {

                // Even total number of elements
                if (m + n) % 2 == 0 {
                    let left_max = l1.max(l2);
                    let right_min = r1.min(r2);

                    return (left_max as f64 + right_min as f64) / 2.0;
                }

                // Odd total number of elements
                return l1.max(l2) as f64;
            }

            // Move binary search towards left
            else if l1 > r2 {
                high = cut1 - 1;
            }

            // Move binary search towards right
            else {
                low = cut1 + 1;
            }
        }

        0.0
    }
}

fn main() {
    let nums1 = vec![1, 3];
    let nums2 = vec![2];

    let result = Solution::find_median_sorted_arrays(nums1, nums2);

    println!("Median: {}", result);
}
