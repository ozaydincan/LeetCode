

struct Solution;

impl Solution {
    pub fn roman_to_int(s:String)->i32{
        let roman_numerals:Vec<char> = s.chars().collect();
        let roman_ten: std::collections::HashMap<char, i32> =std::collections::HashMap::from([('I', 1), ('V', 5), 
        ('X', 10), ('L', 50), ('C', 100), ('D', 500), ('M', 1000)]);
        let mut ans:i32=0; let n = roman_numerals.len();
        for i in 0..n{
            let current_num: i32 = roman_ten[&roman_numerals[i]];
            if i+1 < n{
                let next_num:i32 = roman_ten[&roman_numerals[i+1]];
                if current_num < next_num{
                    ans -= current_num;
                }else {
                    ans += current_num;
                }
        }
            else{
                ans+= current_num;
        }
        }
        
        return ans;
    }
}


fn main() {
    Solution::roman_to_int(("D").to_string());
}


#[cfg(test)]

mod test{
    use super::*;
    
    #[test]
    fn does_it_work(){
        assert_eq!(Solution::roman_to_int("III".to_string()), 3);
        assert_eq!(Solution::roman_to_int("MCMXCIV".to_string()), 1994);
        assert_eq!(Solution::roman_to_int("LVIII".to_string()), 58);
        assert_eq!(Solution::roman_to_int("D".to_string()), 500);
    }
}