
use std::io;

fn is_match(s: String, p: String) -> bool {

    let s = s.as_bytes();
    let p = p.as_bytes();

    let n = s.len();
    let m = p.len();

    // Create DP table
    let mut dp = vec![vec![-1i8; m + 1]; n + 1];

    // Recursive helper function
    fn is_match_rec(
        s: &[u8],
        p: &[u8],
        n: usize,
        m: usize,
        dp: &mut Vec<Vec<i8>>,
    ) -> bool {

        // Base case
        if m == 0 {
            return n == 0;
        }

        // Already calculated?
        if dp[n][m] != -1 {
            return dp[n][m] == 1;
        }

        let ans: bool;

        // Handle '*'
        if p[m - 1] == b'*' {

            // Case 1: Zero occurrences
            let zero = is_match_rec(s, p, n, m - 2, dp);

            // Case 2: One or more occurrences
            let mut one_or_more = false;

            if n > 0 && (p[m - 2] == s[n - 1] || p[m - 2] == b'.') {

                one_or_more = is_match_rec(s, p, n - 1, m, dp);
            }

            ans = zero || one_or_more;

        } else {

            // Handle normal character or '.'
            if n > 0 && (s[n - 1] == p[m - 1] || p[m - 1] == b'.') {

                ans = is_match_rec(s, p, n - 1, m - 1, dp);

            } else {

                ans = false;
            }
        }

        // Save answer
        dp[n][m] = if ans { 1 } else { 0 };

        ans
    }

    is_match_rec(s, p, n, m, &mut dp)
}

fn main() {

    let mut s = String::new();
    let mut p = String::new();

    println!("Enter string:");
    io::stdin().read_line(&mut s).unwrap();

    println!("Enter pattern:");
    io::stdin().read_line(&mut p).unwrap();

    let s = s.trim().to_string();
    let p = p.trim().to_string();

    let result = is_match(s, p);

    println!("Result: {}", result);
}
