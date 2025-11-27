# LeetCode 409: Longest Palindrome

## Problem Description

[https://github.com/zonblade/mentoring-2025/issues/1]
[https://leetcode.com/problems/longest-palindrome/description/]

Given a string `s` which consists of lowercase or uppercase letters, return the length of the **longest palindrome** that can be built with those letters.

Letters are **case sensitive**, for example, `"Aa"` is not considered a palindrome here.

**Example 1:**
```
Input: s = "abccccdd"
Output: 7
Explanation: One longest palindrome that can be built is "dccaccd", whose length is 7.
```

**Example 2:**
```
Input: s = "a"
Output: 1
Explanation: The longest palindrome that can be built is "a", whose length is 1.
```

## Running the Code

```bash
cargo run
```

## Time Complexity
- **O(n)** where n is the length of the input string

## Space Complexity
- **O(k)** where k is the number of unique characters in the string

## Implementation Notes

### Performance Observation

- There's an interesting performance difference between these two approaches (benchmarked in leetcode):

**Slower approach**:
```rust
if count % 2 == 0 {
    longest_palindrome += count;
} else {
    longest_palindrome += count - 1;
    has_odd = true;
}
```

**Faster approach**:
```rust
if count % 2 == 0 {
    for _ in 0..count {
        longest_palindrome += 1;
    }
} else {
    for _ in 0..count - 1 {
        longest_palindrome += 1;
    }
    has_odd = true;
}
```

- Can't index String
