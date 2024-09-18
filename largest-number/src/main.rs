struct Solution;

impl Solution {
    pub fn largest_number(nums: Vec<i32>) -> String {
        let mut str_nums: Vec<String> = nums.into_iter().map(|num| num.to_string()).collect();
        str_nums.sort_by(|a, b| (b.to_string() + a).cmp(&(a.to_string() + b)));

        if str_nums[0] != "0" {
            return str_nums.concat();
        }

        "0".to_string()
    }
}

fn main() {
    let nums = vec![3, 30, 34, 5, 9];
    let largest_num: String = Solution::largest_number(nums);
    println!("The largest num is {:?}", largest_num);
}
