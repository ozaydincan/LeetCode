struct Solution;
impl Solution {
    pub fn find_min_difference(time_points: Vec<String>) -> i32 {
        let mut total_mins: Vec<i32> = time_points
            .iter()
            .map(|times| {
                let parts: Vec<i32> = times
                    .split(':')
                    .map(|lines| lines.parse::<i32>().unwrap())
                    .collect();
                parts[0] * 60 + parts[1]
            })
            .collect();

        total_mins.sort_unstable();

        total_mins
            .windows(2)
            .map(|pair| pair[1] - pair[0])
            .chain(std::iter::once(
                1440 + total_mins[0] - total_mins[total_mins.len() - 1],
            ))
            .min()
            .unwrap()
    }
}
fn main() {
    let times = vec!["23:59".to_string(), "00:00".to_string()];
    let result = Solution::find_min_difference(times);
    println!("{}", result);
}
