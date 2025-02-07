struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn are_almost_equal(s1: String, s2: String) -> bool {
        if s1.len() != s2.len() {
            return false;
        }

        let mut diff_indices = vec![];

        for (i, (c1, c2)) in s1.chars().zip(s2.chars()).enumerate() {
            if c1 != c2 {
                diff_indices.push(i);
            }
        }

        match diff_indices.len() {
            0 => true,
            2 => {
                let (i, j) = (diff_indices[0], diff_indices[1]);
                let s1_bytes = s1.as_bytes();
                let s2_bytes = s2.as_bytes();
                s1_bytes[i] == s2_bytes[j] && s1_bytes[j] == s2_bytes[i]
            }
            _ => false,
        }
    }
}

fn main() {
    todo!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert!(Solution::are_almost_equal(
            "bank".to_string(),
            "kanb".to_string()
        ));
    }

    #[test]
    fn test_2() {
        assert!(!Solution::are_almost_equal(
            "attack".to_string(),
            "defend".to_string()
        ));
    }

    #[test]
    fn test_3() {
        assert!(Solution::are_almost_equal(
            "kelb".to_string(),
            "kelb".to_string()
        ));
    }

    #[test]
    fn test_4() {
        assert!(!Solution::are_almost_equal(
            "abcd".to_string(),
            "abdc".to_string()
        ));
    }
}
