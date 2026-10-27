
use std::collections::HashSet;

impl Solution {
    pub fn is_happy(mut n: i32) -> bool {
        let mut seen = HashSet::new();

        while n != 1 {
            if seen.contains(&n) {
                return false;
            }

            seen.insert(n);

            let mut sum = 0;

            while n > 0 {
                let digit = n % 10;
                sum += digit * digit;
                n /= 10;
            }

            n = sum;
        }

        true
    }
}
