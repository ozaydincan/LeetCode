use std::collections::HashMap;

struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn query_results(_: i32, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut ball_to_color = HashMap::new();

        queries
            .iter()
            .scan(HashMap::new(), |color_count, query| {
                let (ball, color) = (query[0], query[1]);

                if let Some(&prev_color) = ball_to_color.get(&ball) {
                    if let Some(count) = color_count.get_mut(&prev_color) {
                        *count -= 1;
                        if *count == 0 {
                            color_count.remove(&prev_color);
                        }
                    }
                }

                ball_to_color.insert(ball, color);
                *color_count.entry(color).or_insert(0) += 1;

                Some(color_count.len() as i32)
            })
            .collect()
    }
}

fn main() {
    todo!();
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_1() {
        assert_eq!(
            vec![1, 2, 2, 3],
            Solution::query_results(4, vec![vec![1, 4], vec![2, 5], vec![1, 3], vec![3, 4]])
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            vec![1, 2, 2, 3, 4],
            Solution::query_results(
                4,
                vec![vec![0, 1], vec![1, 2], vec![2, 2], vec![3, 4], vec![4, 5]]
            )
        );
    }

    #[test]
    fn test_3() {
        assert_eq!(
            vec![1, 1, 1, 1],
            Solution::query_results(3, vec![vec![1, 2], vec![1, 2], vec![1, 2], vec![1, 2]])
        );
    }
}
