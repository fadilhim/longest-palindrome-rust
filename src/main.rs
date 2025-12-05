// Longest Palindrome, case sensitive
// ref: https://leetcode.com/problems/longest-palindrome/description/

mod utils;

use crate::utils::input::*;
use crate::utils::palindrome::*;
use crate::utils::string::*;

fn main() {
    println!("--------------------------------");
    println!("Count longest palindrome from a word");
    println!("--------------------------------");
    println!("Enter a word: ");
    
    let word: String = get_input();
    let longest_palindrome: i32 = count_longest_palindrome(&word);
    
    println!("Longest palindrome length possible from {} is {} {}", word, longest_palindrome, add_plural(&"character", longest_palindrome));
    println!("--------------------------------");
}
