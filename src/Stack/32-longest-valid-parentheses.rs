
use std::io;

struct Solution;

impl Solution {
    pub fn longest_valid_parentheses(s: String) -> i32 {

        let mut stack: Vec<usize> = vec![usize::MAX];
        let mut ans: usize = 0;

        for (i, ch) in s.chars().enumerate() {

            if ch == '(' {
                stack.push(i);

            } else {
                stack.pop();

                if stack.is_empty() {
                    stack.push(i);

                } else {
                    let length = i - stack[stack.len() - 1];

                    ans = ans.max(length);
                }
            }
        }

        ans as i32
    }
}

fn main() {

    let mut s = String::new();

    println!("Enter parentheses string:");

    io::stdin()
        .read_line(&mut s)
        .expect("Failed to read input");

    let s = s.trim().to_string();

    let result = Solution::longest_valid_parentheses(s);

    println!("Longest Valid Parentheses Length: {}", result);
}
