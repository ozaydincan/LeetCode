use std::collections::HashMap;

#[inline]
fn build_freq_map(s: &str) -> HashMap<char, usize> {
    s.chars().fold(HashMap::new(), |mut acc, c| {
        *acc.entry(c).or_insert(0) += 1;
        acc
    })
}

#[inline]
fn are_maps_equal(m1: &HashMap<char, usize>, m2: &HashMap<char, usize>) -> bool {
    m1 == m2
}

struct Solution;

impl Solution {
    pub fn check_inclusion(s1: String, s2: String) -> bool {
        if s1.len() > s2.len() {
            return false;
        }

        let s1_map = build_freq_map(&s1);
        let mut s2_map = build_freq_map(&s2[..s1.len()]);

        if are_maps_equal(&s1_map, &s2_map) {
            return true;
        }

        s2.chars()
            .zip(s2.chars().skip(s1.len()))
            .any(|(left_char, right_char)| {
                s2_map.entry(left_char).and_modify(|e| *e -= 1);
                if s2_map[&left_char] == 0 {
                    s2_map.remove(&left_char);
                }
                *s2_map.entry(right_char).or_insert(0) += 1;
                are_maps_equal(&s1_map, &s2_map)
            })
    }
}

fn main() {
    let s1: String = String::from("ab");
    let s2: String = String::from("eidbaooo");

    let includes: bool = Solution::check_inclusion(s1, s2);
    println!("Does s2 include s1: {:?}", includes);
}
