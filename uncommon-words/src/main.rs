use std::collections::HashMap;

use std::time::Instant;

struct Solution;

impl Solution {
    pub fn uncommon_from_sentences(s1: String, s2: String) -> Vec<String> {
        let mut words_map =
            s1.split_ascii_whitespace()
                .fold(HashMap::new(), |mut appears, word| {
                    *appears.entry(word).or_insert(0) += 1;
                    appears
                });
        s2.split_ascii_whitespace().for_each(|word| {
            *words_map.entry(word).or_insert(0) += 1;
        });
        words_map
            .into_iter()
            .filter_map(|(word, count): (&str, i32)| {
                if count.abs() == 1 {
                    Some(word.to_string())
                } else {
                    None
                }
            })
            .collect()
    }
}

fn main() {
    let start = Instant::now();
    let s1 = "apple apple".to_string();
    let s2 = "banana".to_string();
    let uncommons = Solution::uncommon_from_sentences(s1, s2);
    println!("{:?}", uncommons);
    let elapse = start.elapsed();
    println!("{:?}", elapse);
}
