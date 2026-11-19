
struct Solution;

impl Solution {
    pub fn first_missing_positive(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        let n = nums.len();

        // Step 1: Har positive number ko uski correct position par rakho.
        for i in 0..n {
            while nums[i] >= 1
                && nums[i] <= n as i32
                && nums[(nums[i] - 1) as usize] != nums[i]
            {
                let correct_index = (nums[i] - 1) as usize;

                nums.swap(i, correct_index);
            }
        }

        // Step 2: Check karo kis position par number missing hai.
        for i in 0..n {
            if nums[i] != (i as i32) + 1 {
                return (i as i32) + 1;
            }
        }

        // Step 3: Agar 1 se n tak sab numbers mojood hain.
        (n as i32) + 1
    }
}

fn main() {
    let nums = vec![3, 4, -1, 1];

    let result = Solution::first_missing_positive(nums);

    println!("First Missing Positive: {}", result);
}
