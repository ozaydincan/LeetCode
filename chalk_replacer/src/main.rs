pub struct Solution;

impl Solution {
    pub fn chalk_replacer(chalk: Vec<i32>, k: i32) -> i32 {
        let mut sum = 0;

        for &chalk_use in &chalk {
            sum += chalk_use;
            if sum > k {
                break;
            }
        }
        let mut k = k % sum;
        for (i, &chalk_use) in chalk.iter().enumerate() {
            if k < chalk_use {
                return i as i32;
            }
            k -= chalk_use;
        }
        0
    }
}
fn main() {
    println!("Hello, world!");
}

#[cfg(test)]

mod test {
    use crate::Solution;

    #[test]

    fn student_test() {
        let result_1 = Solution::chalk_replacer(vec![5, 15], 22);
        let result_2 = Solution::chalk_replacer(vec![3, 4, 1, 2], 25);
        assert_eq!(result_1, 0);
        assert_eq!(result_2, 1);
    }
}
