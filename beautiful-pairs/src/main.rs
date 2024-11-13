use std::vec;

struct Solution;

impl Solution {
    pub fn count_fair_pairs(nums: Vec<i32>, lower: i32, upper: i32) -> i64 {
        let mut nums = nums.clone();
        nums.sort_unstable();
        let mut total: i64 = 0;

        (0..nums.len()).for_each(|i| {
            let min_val = lower - nums[i];
            let max_val = upper - nums[i];

            let low = Self::lower_bin_search(&nums, i + 1, min_val);
            let high = Self::upper_bin_search(&nums, i + 1, max_val);

            total += (high - low) as i64;
        });
        total
    }

    fn lower_bin_search(nums: &[i32], start: usize, target: i32) -> usize {
        let mut left = start;
        let mut right = nums.len();
        while left < right {
            let mid = left + (right - left) / 2;
            if nums[mid] < target {
                left = mid + 1;
            } else {
                right = mid;
            }
        }
        left
    }

    fn upper_bin_search(nums: &[i32], start: usize, target: i32) -> usize {
        let mut left = start;
        let mut right = nums.len();
        while left < right {
            let mid = left + (right - left) / 2;
            if nums[mid] <= target {
                left = mid + 1;
            } else {
                right = mid;
            }
        }
        left
    }
}
fn main() {
    let nums = vec![0, 1, 7, 4, 4, 5];
    let lower = 3;
    let upper = 6;
    println!(
        "The number of beautiful pairs in {:#?} is {}",
        nums,
        Solution::count_fair_pairs(nums.clone(), lower, upper)
    );
}
