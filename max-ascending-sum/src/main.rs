struct Solution;

impl Solution {
    pub fn max_ascending_sum(nums: Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }

        let mut max_sum: i32 = nums[0];
        let mut curr_sum: i32 = nums[0];

        (1..nums.len()).for_each(|i| {
            match nums[i] > nums[i - 1] {
                true => {
                    curr_sum += nums[i];
                }
                false => {
                    if curr_sum > max_sum {
                        max_sum = curr_sum;
                    }
                    curr_sum = nums[i];
                }
            }

            if curr_sum > max_sum {
                max_sum = curr_sum;
            }
        });

        max_sum
    }
}

fn main() {
    let num: Vec<i32> = vec![10, 20, 30, 40, 50];
    println!("The solution is {}", Solution::max_ascending_sum(num));
}
