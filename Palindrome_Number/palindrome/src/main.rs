struct Solution {
}


impl Solution {
    pub fn is_palindrome (x:i32) -> bool{
        let my_num:Vec<char> = x.to_string()
                                    .chars()
                                    .map(|num| num)
                                    .collect();
                                    
        let mut left:usize = 0;
        let mut right:usize= my_num.len()-1;
        while left<=right {
          if my_num[left] != my_num[right]{
            return false;
          }
          left +=1;
          right -=1;
        }
        true
    }
}

#[cfg(test)]

mod tests{
    use super::*;
    #[test]
    fn explore(){
        assert_eq!(Solution::is_palindrome(1234), false);
    }

}

fn main() {
    
}
