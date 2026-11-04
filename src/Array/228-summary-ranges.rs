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
    // Input values
    let nums = vec![2, 3, 4, 7, 9, 10];

    // Function call
    let result = Solution::summary_ranges(nums.clone());

    // Print output
    println!("Input: {:?}", nums);
    println!("Output: {:?}", result);
}
