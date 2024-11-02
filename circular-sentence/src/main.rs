struct Solution;

impl Solution {
    pub fn is_circular_sentence(sentence: String) -> bool {
        let words: Vec<Vec<char>> = sentence
            .split_whitespace()
            .map(|word| word.chars().collect::<Vec<char>>())
            .collect();

        let n: usize = words.len();

        for i in 0..n {
            let curr_last = words[i].len() - 1;
            let current_word_last_char = words[i][curr_last];
            let next_word_first_char = if i == n - 1 {
                words[0][0]
            } else {
                words[i + 1][0]
            };

            if current_word_last_char != next_word_first_char {
                return false;
            }
        }

        true
    }
}
fn main() {
    let sentence = String::from("leetcode exercises sound delightful");
    println!("{:?}", Solution::is_circular_sentence(sentence));
}
