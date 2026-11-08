
use std::io;

fn is_anagram(s: &str, t: &str) -> bool {

    if s.len() != t.len() {
        return false;
    }

    let mut count = [0; 26];

    // Count characters in s
    for ch in s.chars() {
        count[(ch as usize) - ('a' as usize)] += 1;
    }

    // Decrease characters in t
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
