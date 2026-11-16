
use std::collections::HashMap;
use std::io::{self, Read};

struct Solution;

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

fn main() {

    let mut input = String::new();

    io::stdin().read_to_string(&mut input).unwrap();

    let mut iter = input.split_whitespace();

    let s = iter.next().unwrap().to_string();

    let n: usize = iter.next().unwrap().parse().unwrap();

    let mut words = Vec::new();

    for _ in 0..n {
        words.push(iter.next().unwrap().to_string());
    }

    let result = Solution::find_substring(s, words);

    println!("Starting indices: {:?}", result);
}
