# 30. Substring with Concatenation of All Words

**LeetCode Problem:** [30. Substring with Concatenation of All Words](https://leetcode.com/problems/substring-with-concatenation-of-all-words/)

## # Intuition

We need to find all starting indices where the substring contains every word from the `words` array exactly once, in any order.

Since all words have the same length, we can use a Sliding Window and a HashMap to count words efficiently.

## # Approach

1. First, calculate the length of each word and the total number of words.
2. Create a `HashMap` called `word_count` to store how many times each word appears in the `words` vector.
3. Try different starting positions from `0` to `word_length - 1`.
4. Use two pointers, `left` and `right`, to maintain a sliding window.
5. Extract a word of `word_length` from the current `right` position.
6. If the extracted word does not exist in `word_count`, clear the `seen` map and reset the window.
7. If the word exists, increase its count in `seen`.
8. If a word appears more times than allowed, move the `left` pointer forward and remove extra words.
9. When the window contains all required words, add the `left` index to the answer.
10. Finally, return the answer vector.

## # Complexity

- **Time complexity:** \(O(n \times m)\)

  Where `n` is the length of string `s` and `m` is the length of each word. Creating and hashing each substring takes up to \(O(m)\).

- **Space complexity:** \(O(k \times m)\)

  Where `k` is the number of unique words stored in the HashMaps and `m` is the length of each word.

## # Code

```rust
use std::collections::HashMap;

impl Solution {
    pub fn find_substring(s: String, words: Vec<String>) -> Vec<i32> {

        let mut ans = Vec::new();

        if words.is_empty() {
            return ans;
        }

        let word_length = words[0].len();
        let total_words = words.len();

        if s.len() < word_length * total_words {
            return ans;
        }

        let mut word_count: HashMap<String, usize> = HashMap::new();

        // Step 1: Count words
        for word in &words {
            *word_count.entry(word.clone()).or_insert(0) += 1;
        }

        // Step 2: Try different starting positions
        for i in 0..word_length {

            let mut left = i;
            let mut count = 0;

            let mut seen: HashMap<String, usize> = HashMap::new();

            // Step 3: Move through string
            let mut right = i;

            while right + word_length <= s.len() {

                let word = s[right..right + word_length].to_string();

                // Step 4: Check if word exists
                if !word_count.contains_key(&word) {

                    seen.clear();
                    count = 0;
                    left = right + word_length;
                    right += word_length;

                    continue;
                }

                *seen.entry(word.clone()).or_insert(0) += 1;
                count += 1;

                // Step 5: Remove extra words
                while seen[&word] > word_count[&word] {

                    let left_word =
                        s[left..left + word_length].to_string();

                    if let Some(value) = seen.get_mut(&left_word) {
                        *value -= 1;
                    }

                    count -= 1;
                    left += word_length;
                }

                // Step 6: All words found
                if count == total_words {

                    ans.push(left as i32);

                    let left_word =
                        s[left..left + word_length].to_string();

                    if let Some(value) = seen.get_mut(&left_word) {
                        *value -= 1;
                    }

                    count -= 1;
                    left += word_length;
                }

                right += word_length;
            }
        }

        ans
    }
}
```
