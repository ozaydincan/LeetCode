use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn lemonade_change(bills: std::vec::Vec<i32>) -> bool{
        let mut bill_map:HashMap<i32,i32> = HashMap::new();
        for i in bills{
            if i == 5{
                *bill_map.entry(i).or_insert(0) += 1;
            }
            else if (i == 10)&& (bill_map.get(&5) > Some(&0)) {
                *bill_map.entry(i).or_insert(0) += 1;
                *bill_map.entry(5).or_insert(0) -= 1;
            }
            else if i == 20{
                if (bill_map.get(&10) >= Some(&1)) && (bill_map.get(&5) >= Some(&1)){
                    *bill_map.entry(5).or_insert(0) -= 1;
                    *bill_map.entry(10).or_insert(0) -= 1;
                    *bill_map.entry(20).or_insert(0) += 1;
                }
                else if bill_map.get(&5) >= Some(&3) {
                    *bill_map.entry(5).or_insert(0) -= 3;
                    *bill_map.entry(20).or_insert(0) += 1;

                }else{
                    return false;
                }
            }
            else{
                return false;
            }
        }

        return true;
    }
}


fn main() {
    println!("{}",Solution::lemonade_change(Vec::from([5,5,10,10,20])));
}


#[cfg(test)]

mod test{

    use super::*;

    #[test]
    fn checks(){
        assert_eq!(Solution::lemonade_change(Vec::from([5,5,5,10,20])), true);
        assert_eq!(Solution::lemonade_change(Vec::from([5,5,10,10,20])), false);
        
    }
}