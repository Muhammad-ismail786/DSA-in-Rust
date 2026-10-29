# 205. Isomorphic Strings

## Intuition
We need to check whether characters in strings `s` and `t` follow the same pattern. Each character must consistently map to one character in the other string.

## Approach
1. If the lengths of both strings are different, return `false`.
2. Convert both strings into byte slices using `as_bytes()`.
3. Create two arrays, `map_s` and `map_t`, each containing 256 integers initialized to zero.
4. Loop through both strings using index `i`.
5. Compare the last-seen positions of the current characters. If they differ, return `false`.
6. Update both arrays with `i + 1` to record the current position.
7. If all characters follow the same pattern, return `true`.

## Complexity
- Time complexity: `O(n)`
- Space complexity: `O(1)`

## Code

```rust
impl Solution {
    pub fn is_isomorphic(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }

        let s = s.as_bytes();
        let t = t.as_bytes();

        let mut map_s = [0usize; 256];
        let mut map_t = [0usize; 256];

        for i in 0..s.len() {
            if map_s[s[i] as usize] != map_t[t[i] as usize] {
                return false;
            }

            map_s[s[i] as usize] = i + 1;
            map_t[t[i] as usize] = i + 1;
        }

        true
    }
}
```
