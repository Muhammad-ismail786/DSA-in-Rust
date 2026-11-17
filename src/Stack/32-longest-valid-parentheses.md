
# LeetCode 32: Longest Valid Parentheses

## # Intuition

We are given a string containing only opening `(` and closing `)` parentheses.

We need to find the length of the longest valid parentheses substring.

We can solve this problem using a Stack.

Instead of storing parentheses, we store their indexes in the stack.

We initialize the stack with `usize::MAX` as a starting boundary marker.

## # Approach

1. Create a stack using `Vec<usize>` and initialize it with `usize::MAX`.
2. Initialize an answer variable with `0`.
3. Traverse the string using `chars().enumerate()`.
4. If the current character is `(`, push its index into the stack.
5. Otherwise, pop the top index from the stack.
6. If the stack becomes empty, push the current index as a new boundary.
7. Otherwise, calculate the valid substring length using the current index minus the top index.
8. Update the maximum answer using `max()`.
9. Convert the final answer into `i32` and return it.

## # Complexity

- Time complexity: `O(n)`

  We traverse the string only once.

- Space complexity: `O(n)`

  In the worst case, the stack can store up to `n` indexes.

## # Code

```rust
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
```
