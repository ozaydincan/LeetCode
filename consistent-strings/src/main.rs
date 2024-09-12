use std::collections::HashSet;

struct Solution;

impl Solution {
    pub fn count_consistent_strings(allowed: String, words: Vec<String>) -> i32 {
        let mut count = 0;
        let allowed_set: HashSet<_> = allowed.chars().collect::<HashSet<_>>();
        words.iter().for_each(|word| {
            if word.chars().all(|ch| allowed_set.contains(&ch)) {
                count += 1;
            }
        });
        count
    }
}

fn main() {
    let words: Vec<String> = vec![
        "ad".to_string(),
        "bd".to_string(),
        "aaab".to_string(),
        "baa".to_string(),
        "badab".to_string(),
    ];
    let allowed = "ab".to_string();
    let allowed_count = Solution::count_consistent_strings(allowed, words);
    println!("{allowed_count} allowed strings");
}

#[cfg(test)]

mod test {
    use crate::Solution;
    #[test]
    fn test_1() {
        let words: Vec<String> = vec![
            "ad".to_string(),
            "bd".to_string(),
            "aaab".to_string(),
            "baa".to_string(),
            "badab".to_string(),
        ];
        let allowed = "ab";
        assert_eq!(
            Solution::count_consistent_strings(allowed.to_string(), words),
            2
        );
    }
}
