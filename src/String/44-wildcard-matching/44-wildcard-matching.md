
# 44. Wildcard Matching

**Difficulty:** Hard

**Language:** Rust

**Approach:** Greedy Algorithm

## Intuition

We need to check whether the given pattern `p` matches the entire string `s`.

There are two special characters:

- `?` matches any single character.
- `*` matches any sequence of characters, including an empty sequence.

We use a Greedy Approach to solve this problem.

We keep track of the last `*` position and use it to match additional characters whenever a mismatch occurs.

## Approach

1. Initialize two pointers `i` and `j` to traverse strings `s` and `p`.
2. Convert both strings into byte slices using `as_bytes()`.
3. Initialize `star = -1` to store the last position of `*`.
4. Initialize `match_index = 0` to store the position in string `s` where `*` starts matching.
5. If characters match or the pattern contains `?`, increment both pointers.
6. If the pattern contains `*`, save its position in `star` and the current string position in `match_index`.
7. If a mismatch occurs and a previous `*` exists, let `*` match one additional character and continue checking.
8. If no previous `*` exists, return `false`.
9. After processing the string, skip any remaining `*` characters in the pattern.
10. Return `true` if the entire pattern has been matched; otherwise, return `false`.

## Complexity

- **Time Complexity:** O(n + m) — Linear amortized time using greedy matching.
- **Space Complexity:** O(1) — Only a constant number of variables are used.

## Code

### Rust

\`\`\`rust
impl Solution {
    pub fn is_match(s: String, p: String) -> bool {

        let s = s.as_bytes();
        let p = p.as_bytes();

        let mut i = 0;
        let mut j = 0;

        let mut star: isize = -1;
        let mut match_index = 0;

        while i < s.len() {

            if j < p.len() && (s[i] == p[j] || p[j] == b'?') {

                i += 1;
                j += 1;

            } else if j < p.len() && p[j] == b'*' {

                star = j as isize;
                match_index = i;
                j += 1;

            } else if star != -1 {

                j = star as usize + 1;
                match_index += 1;
                i = match_index;

            } else {

                return false;
            }
        }

        while j < p.len() && p[j] == b'*' {
            j += 1;
        }

        j == p.len()
    }
}
\`\`\`
