use std::vec;

struct Solution;


impl Solution {
    pub fn max_points(points: Vec<Vec<i32>>) -> i64 {
        let max = points.into_iter().map(|row: Vec<i32>|{
            row.into_iter().enumerate().map(|(colid, val)|
                (val - (colid as i32)) as i32)
                .max()
                .unwrap_or(0) as i64
        })
        .sum();
        return max;     
    }
}
fn main() {
    let arr:Vec<Vec<i32>> = vec![vec![1,2,3],vec![1,5,1],vec![3,1,1]];
    println!("{}", Solution::max_points(arr));
}
