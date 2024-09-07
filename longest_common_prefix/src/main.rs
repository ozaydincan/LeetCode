struct Solution;

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>)-> String{
        if strs.len() == 0{
            return "".to_string();
        }
        let mut common_prefix:String = strs[0].clone(); 
        strs.iter().for_each(|s|{
            while !s.starts_with(&common_prefix) {
                if common_prefix.is_empty() {
                     break;
                }
                common_prefix.pop();
            }
        });
        return common_prefix; 
    }
}

use std::vec::*;

fn main() {

   let strs = vec![
    "flower".to_string(),
    "flow".to_string(),
    "flight".to_string(),
   ];

   assert_eq!(Solution::longest_common_prefix(strs), String::from("fl"));

}

#[cfg(test)]

mod test{
    use super::*;
    #[test]
    fn it_works(){
        let strs = vec![
            "flower".to_string(),
            "flow".to_string(),
            "flight".to_string(),
           ];
        
    assert_eq!(Solution::longest_common_prefix(strs), String::from("fl")); }
}


