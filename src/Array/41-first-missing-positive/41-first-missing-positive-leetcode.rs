
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
