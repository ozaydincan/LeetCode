use itoa::Buffer;

struct Solution;
impl Solution {
    pub fn get_lucky(s: String, k: i32) -> i32 {
        let mut num_str: String = Solution::stoi(&s);
        for _i in 0..k {
            num_str = Solution::sum_digits(num_str.parse::<u64>().unwrap());
        }
        num_str.parse::<i32>().unwrap()
    }
    fn sum_digits(x: u64) -> String {
        ((x % 10)
            + (0..)
                .scan(x, |num, _| {
                    *num /= 10;
                    Some(*num)
                })
                .take_while(|num| *num > 0)
                .map(|num| num % 10)
                .sum::<u64>())
        .to_string()
    }
    fn stoi(s: &str) -> String {
        let mut num_str = String::new();
        let mut buffer = Buffer::new();

        for ch in s.chars() {
            let num_ascii = ch as u8 - 96;
            num_str.push_str(buffer.format(num_ascii));
        }

        num_str
    }
}
fn main() {
    let k: i32 = 1;
    let s = "iiii".to_string();
    let num: i32 = Solution::get_lucky(s, k);
    println!("{num}");
}
