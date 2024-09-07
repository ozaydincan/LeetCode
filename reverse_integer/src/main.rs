struct Solution;

impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let x_str: String = x.abs().to_string().chars().rev().collect::<String>();
        println!("{x_str}");
        let rev_int: i32 = x_str.parse::<i32>().unwrap_or(0);
        if x < 0 {
            -rev_int
        } else {
            rev_int
        }
    }
}

#[allow(arithmetic_overflow)]
fn main() {
    println!("Hello, world!");

    let rev = Solution::reverse(-123);
    println!("{rev}")
}
