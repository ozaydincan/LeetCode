struct Solution;

impl Solution {
    pub fn count_days(days: i32, meetings: Vec<Vec<i32>>) -> i32 {
        let mut day_tracker: Vec<i32> = vec![0; days.try_into().unwrap()];
        for nums in meetings.iter() {
            for i in nums[0] - 1..nums[1] {
                day_tracker[i as usize] = 1;
            }
        }
        day_tracker
            .iter()
            .filter(|&n| *n == 0)
            .count()
            .try_into()
            .unwrap()
    }
}

fn main() {
    let days = 10;
    let meetings: Vec<Vec<i32>> = vec![vec![5, 7], vec![1, 3], vec![9, 10]];
    assert_eq!(2, Solution::count_days(days, meetings));
}
