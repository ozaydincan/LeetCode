use std::time::Instant;

struct Solution;

impl Solution {
    pub fn xor_queries(arr: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut xors: Vec<i32> = vec![0; arr.len() + 1];

        for i in 0..arr.len() {
            xors[i + 1] = xors[i] ^ arr[i];
        }

        let mut results: Vec<i32> = Vec::new();
        for query in queries {
            let left = query[0] as usize;
            let right = query[1] as usize;
            let result = xors[right + 1] ^ xors[left];
            results.push(result);
        }

        results
    }
}

fn main() {
    let start = Instant::now();

    let arr = vec![1, 3, 4, 8];
    let queries = vec![vec![0, 1], vec![1, 2], vec![0, 3], vec![3, 3]];

    let results = Solution::xor_queries(arr, queries);

    println!("{:?}", results);

    let run_time = start.elapsed();
    println!("{:?}", run_time);
}
