use std::collections::HashMap;

pub fn count_longest_palindrome(word: &str) -> i32 {
    let char_count: HashMap<char, i32> = count_char(word);
    let mut longest_palindrome_length: i32 = 0;
    let mut has_odd = false;

    for (_, count) in char_count {
        if count % 2 == 0 {
            for _ in 0..count {
                longest_palindrome_length += 1;
            }
        } else {
            // formula to count odd char is word.length - 1
            // and we only want to add 1 char count for the odd char(s)
            for _ in 0..count - 1 {
                longest_palindrome_length += 1;
            }
            has_odd = true;
        }
    }

    if has_odd {
        longest_palindrome_length += 1;
    }

    return longest_palindrome_length;
}

fn count_char(word: &str) -> HashMap<char, i32> {
    let mut char_count = HashMap::new();
    for char in word.chars() {
        char_count.entry(char).and_modify(|count| *count += 1).or_insert(1);
    }

    char_count
}