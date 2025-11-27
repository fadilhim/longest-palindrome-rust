// Longest Palindrome, case sensitive
// ref: https://leetcode.com/problems/longest-palindrome/description/

use std::collections::HashMap;

fn main() {
    let word = "abccccdd";
    let longest_palindrome = build_longest_palindrome(word);
    println!("Longest palindrome from {}: {}", word, longest_palindrome);
}

fn build_longest_palindrome(word: &str) -> i32 {
    let mut char_count = HashMap::new();
    let mut longest_palindrome: i32 = 0;

    for char in word.chars() {
        char_count.entry(char).and_modify(|count| *count += 1).or_insert(1);
    }

    let mut has_odd = false;

    for (_, count) in char_count {
        if count % 2 == 0 {
            for _ in 0..count {
                longest_palindrome += 1;
            }
        } else {
            // only add the most even number of characters
            for _ in 0..count - 1 {
                longest_palindrome += 1;
            }
            has_odd = true;
        }
    }

    if has_odd {
        longest_palindrome += 1;
    }

    return longest_palindrome;
}