
use std::collections::HashMap;

impl Solution {
    pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
        let mut mp: HashMap<i32, usize> = HashMap::new();

        for (i, &n) in nums.iter().enumerate() {
            if let Some(&previous_index) = mp.get(&n) {
                let distance = i.abs_diff(previous_index);

                if distance <= k as usize {
                    return true;
                }
            }

            mp.insert(n, i);
        }

        false
    }
}
