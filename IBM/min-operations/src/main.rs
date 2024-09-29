struct Solution;

impl Solution {
    pub fn min_operations(nums: Vec<i32>, queries: Vec<i32>) -> Vec<i64> {
        let mut nums = nums;

        nums.sort_unstable();
        let length = nums.len();
        let mut prefix_sum = vec![0i64; length + 1];
        for i in 1..=length {
            prefix_sum[i] = prefix_sum[i - 1] + nums[i - 1] as i64;
        }

        queries
            .iter()
            .map(|&q| {
                let idx = Solution::binary_search(&nums, q as i64, 0, length);

                let left_count = idx as i64;
                let right_count = (length - idx) as i64;

                let left_oper = left_count * q as i64 - prefix_sum[idx];
                let right_oper = prefix_sum[length] - prefix_sum[idx] - right_count * q as i64;

                left_oper + right_oper
            })
            .collect()
    }
    fn binary_search(nums: &[i32], target: i64, low: usize, high: usize) -> usize {
        if low >= high {
            return low;
        }

        let m = low + (high - low) / 2;
        if nums[m] < target as i32 {
            Solution::binary_search(nums, target, m + 1, high)
        } else {
            Solution::binary_search(nums, target, low, m)
        }
    }
}

fn main() {
    let nums = vec![3, 1, 6, 8];
    let queries = vec![1, 5];

    let output = Solution::min_operations(nums, queries);
    println!("{:?}", output);
}

#[cfg(test)]

mod test {
    use crate::Solution;

    #[test]

    fn test_1() {
        let nums: Vec<i32> = vec![3, 1, 6, 8];
        let queries: Vec<i32> = vec![1, 5];

        assert_eq!(Solution::min_operations(nums, queries), vec![14, 10]);
    }

    #[test]

    fn test_2() {
        let nums: Vec<i32> = vec![2, 9, 6, 3];
        let queries: Vec<i32> = vec![10];

        assert_eq!(Solution::min_operations(nums, queries), vec![20]);
    }
}
