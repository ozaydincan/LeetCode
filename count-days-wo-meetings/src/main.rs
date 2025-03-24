struct Solution;

impl Solution {
    pub fn count_days(days: i32, mut meetings: Vec<Vec<i32>>) -> i32 {
        if meetings.is_empty() {
            return days;
        }

        meetings.sort_by_key(|m| m[0]);

        days - meetings
            .into_iter()
            .fold(Vec::new(), |mut acc: Vec<Vec<i32>>, m| {
                if let Some(last) = acc.last_mut() {
                    if m[0] <= last[1] + 1 {
                        last[1] = last[1].max(m[1]);
                        return acc;
                    }
                }
                acc.push(m);
                acc
            })
            .iter()
            .map(|m| m[1] - m[0] + 1)
            .sum::<i32>()
    }
}

fn main() {
    let days = 10;
    let meetings: Vec<Vec<i32>> = vec![vec![5, 7], vec![1, 3], vec![9, 10]];
    assert_eq!(2, Solution::count_days(days, meetings));
}
