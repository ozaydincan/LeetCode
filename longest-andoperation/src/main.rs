use std::time::Instant;
struct Solution;

impl Solution {
    pub fn longest_subarray(nums: Vec<i32>) -> i32 {
        let mut max_val = 0;
        let mut max_size = 0;
        let mut current_size = 0;

        nums.iter().for_each(|&num| match num {
            n if n > max_val => {
                max_val = n;
                current_size = 1;
                max_size = current_size;
            }
            n if n == max_val => {
                current_size += 1;
                max_size = std::cmp::max(max_size, current_size);
            }
            n if n == max_val => current_size += 1,
            _ => current_size = 0,
        });
        max_size
    }
}

fn main() {
    let start = Instant::now();
    let nums: Vec<i32> = vec![1, 2, 3, 3, 2, 2];
    let max_subarr_size = Solution::longest_subarray(nums);
    println!("{max_subarr_size}");
    let run_time = start.elapsed();
    println!("Ran in: {:?}", run_time);
}

#[cfg(test)]

mod test {
    use crate::Solution;

    #[test]
    fn example_1() {
        let nums: Vec<i32> = vec![1, 2, 3, 3, 2, 2];
        assert_eq!(Solution::longest_subarray(nums), 2);
    }
    #[test]
    fn example_2() {
        let nums: Vec<i32> = vec![1, 2, 3, 4];
        assert_eq!(Solution::longest_subarray(nums), 1);
    }
}
