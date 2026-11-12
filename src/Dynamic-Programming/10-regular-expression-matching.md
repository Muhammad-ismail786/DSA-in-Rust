# 10. Regular Expression Matching

**Difficulty:** Hard

**Topics:** String, Dynamic Programming, Recursion, Memoization

## Intuition

Initially, I thought of solving this problem using recursion by comparing the characters of the string `s` and pattern `p` from the end.

There are two special characters:

- `.` matches any single character.
- `*` matches zero or more occurrences of the preceding character.

Simple recursion can calculate the same subproblems multiple times, which can lead to Time Limit Exceeded.

To avoid repeated calculations, I used **Memoization (Dynamic Programming)** to store previously calculated results in a 2D DP array.

## Approach

1. Convert strings `s` and `p` into byte slices using `as_bytes()`.
2. Calculate the lengths of both strings.
3. Create a 2D DP array initialized with `-1`.
4. Use a recursive helper function `is_match_rec()` to solve the problem.
5. If the pattern becomes empty, return true only if the string is also empty.
6. If the last pattern character is `*`, handle two cases:
   - **Zero occurrences:** Ignore `*` and its preceding character by moving `m - 2`.
   - **One or more occurrences:** If the preceding character matches the current string character (or is `.`), move `n - 1` while keeping `m` unchanged.
7. If the last pattern character is a normal character or `.`, compare it with the last character of the string and move both backward.
8. Store the calculated result in the DP array to avoid repeated calculations.

## Complexity

- **Time complexity:** \(O(n \times m)\)
- **Space complexity:** \(O(n \times m)\)

Where:
- `n` = Length of string `s`.
- `m` = Length of pattern `p`.

## Rust Code

```rust
impl Solution {
    pub fn is_match(s: String, p: String) -> bool {

        let s = s.as_bytes();
        let p = p.as_bytes();

        let n = s.len();
        let m = p.len();

        let mut dp = vec![vec![-1i8; m + 1]; n + 1];

        fn is_match_rec(
            s: &[u8],
            p: &[u8],
            n: usize,
            m: usize,
            dp: &mut Vec<Vec<i8>>,
        ) -> bool {

            if m == 0 {
                return n == 0;
            }

            if dp[n][m] != -1 {
                return dp[n][m] == 1;
            }

            let ans: bool;

            if p[m - 1] == b'*' {

                // Zero occurrences
                let zero = is_match_rec(s, p, n, m - 2, dp);

                // One or more occurrences
                let mut one_or_more = false;

                if n > 0 && (p[m - 2] == s[n - 1] || p[m - 2] == b'.') {
                    one_or_more = is_match_rec(s, p, n - 1, m, dp);
                }

                ans = zero || one_or_more;

            } else {

                if n > 0 && (s[n - 1] == p[m - 1] || p[m - 1] == b'.') {
                    ans = is_match_rec(s, p, n - 1, m - 1, dp);
                } else {
                    ans = false;
                }
            }

            dp[n][m] = if ans { 1 } else { 0 };

            ans
        }

        is_match_rec(s, p, n, m, &mut dp)
    }
}
```

## Example

**Input:**

```text
s = "aab"
p = "c*a*b"
```

**Output:**

```text
true
```

**Explanation:**

- `c*` matches zero occurrences of `c`.
- `a*` matches two occurrences of `a`.
- `b` matches `b`.

Therefore, the complete pattern matches the string.
