struct Solution;

impl Solution {
    pub fn three_sum_closest(nums: Vec<i32>, target: i32) -> i32 {
        let mut diff = i32::MAX;
        let mut nums = nums.clone();
        let length = nums.len();
        nums.sort();

        nums.iter().enumerate().for_each(|(i, &num)| {
            if i > 0 && nums[i] == nums[i - 1] {
                return;
            }

            let (mut low, mut high) = (i + 1, length - 1);
            while low < high {
                let sum = num + nums[low] + nums[high];
                let current_diff = (target - sum).abs();

                if current_diff < (target - diff).abs() {
                    diff = sum;
                }

                if sum < target {
                    low += 1;
                } else {
                    high -= 1;
                }
            }
        });

        diff
    }
}
fn main() {
    let nums: Vec<i32> = vec![-1, 2, 1, -4];
    let target = 1;
    println!(
        "The closest sum in array {:?} to {target} is {:?}",
        nums.clone(.clone),
        Solution::three_sum_closest(nums, target)
    );
}
