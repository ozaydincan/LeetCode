struct Solution;

impl Solution {
    pub fn shortest_palindrome(s: String) -> String {
        if s.is_empty() {
            return s;
        }

        let chars: Vec<char> = s.chars().collect();
        let mut right = chars.len() - 1;

        while right > 0 {
            let mut left = 0;
            let mut valid = true;
            while left < right {
                if chars[left] != chars[right - left] {
                    valid = false;
                    break;
                }
                left += 1;
            }
            if valid {
                break;
            }
            right -= 1;
        }

        let mut palindrome = chars[right + 1..].iter().rev().collect::<String>();
        palindrome.push_str(&s);
        palindrome
    }
}

fn main() {
    let s = "abcd".to_string();
    let pal = Solution::shortest_palindrome(s);
    println!("{pal}");
}
