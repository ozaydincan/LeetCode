

struct Solution;
use std::{cmp::max, i32};
impl Solution {
    pub fn max_distance(arrays:Vec<Vec<i32>>)-> i32{
        let mut min_int:i32 = arrays[0][0];
        let mut max_int:i32 = *arrays[0].last().unwrap();
        let mut max_distance:i32 = 0;
        for i in arrays.iter().skip(1){
            let curr_last = *i.last().unwrap();
            let curr_first = i[0];
            max_distance = max(max_distance, max_int-curr_first);
            max_distance= max(max_distance, curr_last - min_int);
            if curr_first < min_int{
                min_int = i[0];
            }
            if curr_last > max_int{
                max_int = curr_last;
            }
        }
        return max_distance;
    }
}


fn main() {
    println!("{}", Solution::max_distance(vec![vec![-1,1],vec![-3,1,4],vec![-2,-1,0,2]]));
}
#[cfg(test)]

mod test{
    use super::*;

    #[test]

    fn works() {
        assert_eq!(Solution::max_distance(vec![vec![1,2,3],vec![4,5],vec![1,2,3]]), 4);
        assert_eq!(Solution::max_distance(vec![vec![1],vec![1]]), 0);
    }
}
