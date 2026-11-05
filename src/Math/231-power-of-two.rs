struct Solution;

impl Solution {

    fn is_power_of_two(n: i32) -> bool {

        if n <= 0 {
            return false;
        }

        let mut n = n;

        while n % 2 == 0 {
            n = n / 2;
        }

        n == 1
    }
}

fn main() {

    // Input data
    let n: i32 = 16;

    // Function call
    let result = Solution::is_power_of_two(n);

    // Print output
    println!("Input: {}", n);
    println!("Is Power Of Two: {}", result);
}
