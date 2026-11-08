use std::io;

fn is_anagram(s: String, t: String) -> bool {
    if s.len() != t.len() {
        return false;
    }

    let mut count = [0; 26];

    for ch in s.chars() {
        let index = (ch as usize) - ('a' as usize);
        count[index] += 1;
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

    let s = s.trim().to_string();
    let t = t.trim().to_string();

    let result = is_anagram(s, t);

    println!("Output: {}", result);
}
