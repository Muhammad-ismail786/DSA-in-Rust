
# 242. Valid Anagram

## Intuition

The main idea is to check whether both strings contain the same characters with the same frequency.

- If the lengths of both strings are different, return false.
- Create an array of size 26 to store character frequencies.
- Increase the frequency of each character in string `s`.
- Decrease the frequency of each character in string `t`.
- If any character's frequency becomes negative, return false.
- Otherwise, return true.

## Approach

1. First, check whether the lengths of both strings are equal.
2. Create a mutable integer array `count[26]` initialized with zeros.
3. Traverse string `s` and increase the frequency of each character.
4. Traverse string `t` and decrease the frequency of each character.
5. If any frequency becomes negative, return false.
6. If all characters are processed successfully, return true.

## Complexity

- Time complexity: O(n)

  We traverse both strings once, where n is the length of the strings.

- Space complexity: O(1)

  We use a fixed-size array of 26 elements.

## Code

```rust
use std::io;

fn is_anagram(s: &str, t: &str) -> bool {

    if s.len() != t.len() {
        return false;
    }

    let mut count = [0; 26];

    for ch in s.chars() {
        count[(ch as usize) - ('a' as usize)] += 1;
    }

    for ch in t.chars() {
        let index = (ch as usize) - ('a' as usize);

        count[index] -= 1;

        if count[index] < 0 {
            return false;
        }
    }

    true
}

fn main() {

    let mut s = String::new();
    let mut t = String::new();

    println!("Enter first string:");
    io::stdin().read_line(&mut s).unwrap();

    println!("Enter second string:");
    io::stdin().read_line(&mut t).unwrap();

    let s = s.trim();
    let t = t.trim();

    if is_anagram(s, t) {
        println!("Output: true");
    } else {
        println!("Output: false");
    }
}
```

## Example 1

Input:
```text
anagram
nagaram
```

Output:
```text
true
```

## Example 2

Input:
```text
rat
car
```

Output:
```text
false
```
