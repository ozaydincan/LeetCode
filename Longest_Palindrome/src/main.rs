struct Solution{}

impl Solution { 
    pub fn longest_palindrome(s:String) -> String{
        let n: isize = s.len() as isize;
        let mut start:isize = 0; let mut end: isize = 1;    
        let mut low:isize; let mut hi:isize;
 
        for i in 0..n{
            low = i-1;
            hi = i;
        
         while low >= 0 && hi <n && s.chars().nth(low as usize) == s.chars().nth(hi as usize){
             if (hi - low +1 ) > end{
                 start = low;
                 end = hi -low+1;
             }
             low -=1;
             hi += 1;
         }
         low = i-1;
         hi = i+1;
 
         while low >= 0 && hi <n && s.chars().nth(low as usize) == s.chars().nth(hi as usize){
             if (hi - low +1 ) >= end{
                 start = low;
                 end = hi -low+1;
             }
             low -= 1;
             hi += 1;
         }
             
        }
        /*start = (start as usize);
        end = (end as usize);*/
        return s[start as usize..(end+start) as usize].to_string();
     }
    
}


fn main() {
    use std::time::Instant;
    let now = Instant::now();
    let my_str:String = String::from("cbbd");
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
    println!("{}",Solution::longest_palindrome(my_str));
}

#[cfg(test)]

mod test{
    use super::*;
    
    #[test]
    fn it_works(){
        assert_eq!(Solution::longest_palindrome("abbcaba".to_string()), "aba".to_string());
    }
}   
