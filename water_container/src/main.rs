use std::{cmp::min, cmp::max};

struct Solution;

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut left:usize = 0;
        let mut right:usize = height.len()-1;
        let mut area:i32 = 0;
        while left < right {
            area = max(area, (right-left) as i32 * min(height[left], height[right]));
            if height[left] < height[right]{
                left +=1;
            }else {
                right -=1;
            }
        }
        return area;
    }
}


fn main() {
    let v = vec![1,1];
    print!("{}", Solution::max_area(v));
}

#[cfg(test)]

mod test{

    use super::*;

    #[test]
    fn works(){
        assert_eq!(Solution::max_area(vec![1,8,6,2,5,4,8,3,7]), 49);
        assert_eq!(Solution::max_area(vec![1,1]),1);
    }
}
