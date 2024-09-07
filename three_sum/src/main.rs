struct Solution;

impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut triplets:Vec<Vec<i32>> = Vec::new();
        let mut mid:usize;
        let nums_sorted: Vec<i32> = {let mut n = nums;n.sort(); n};
        for i in 0 .. nums_sorted.len(){
            if i > 0 && nums_sorted[i] == nums_sorted[i-1]{
                continue;
            }
            mid = i+1;
            let mut right = nums_sorted.len()-1;
            while mid < right {
                let sum = nums_sorted[i]+nums_sorted[mid]+nums_sorted[right];
                if sum == 0{
                   triplets.push(vec![nums_sorted[i],nums_sorted[mid], nums_sorted[right]]);
                   mid += 1;
                   right -=1;
                   while mid < right && nums_sorted[mid] == nums_sorted[mid - 1] {
                    mid += 1;
                }
                while mid < right && nums_sorted[right] == nums_sorted[right + 1] {
                    right -= 1;
                }
                }
                else if sum < 0{
                    mid += 1;
                }
                else if sum > 0 {
                    right -= 1;
                }
            
            }
        }
        return triplets;
    }
}


fn main() {
    println!("Hello, world!");
}
