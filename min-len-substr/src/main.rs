struct Solution;

impl Solution {
    pub fn min_length(s: String) -> i32 {
        let str_vec: Vec<char> = s.chars().collect();
        let mut count: i32 = 0;
        let in_len: i32 = s.len() as i32;
        println!("Initial len: {in_len}");
        for ref mut i in 0..str_vec.len() {
            if str_vec[*i] == 'A' && str_vec[*i + 1] == 'B' {
                count += 2;
                *i += 1;
                println!("count: {count}");
            } else if str_vec[*i] == 'C' && str_vec[*i + 1] == 'D' {
                count += 2;
                *i += 1;
                println!("count: {count}");
            }
        }
        in_len - count
    }
}

fn main() {
    let s = String::from("ABFCACDB");
    let new_len = Solution::min_length(s);
    println!("new length is: {new_len}");
}
