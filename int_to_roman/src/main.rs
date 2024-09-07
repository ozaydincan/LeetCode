
struct Solution;
impl Solution {
    pub fn int_to_roman(num: i32) -> String{
        let mut original:i32 = num;
        let mut roman:String = String::new();
        if original >= 1000 {
            roman.push_str(&"M".repeat((original / 1000) as usize));
            original -= 1000 * (original / 1000);
        } if original >= 900 {
            roman.push_str("CM");
            original -= 900;
        }if original >= 500 {
            roman.push_str("D");
            original -= 500;
        }  if original >= 400 {
            roman.push_str("CD");
            original -= 400;
        }  if original >= 100 {
            roman.push_str(&"C".repeat((original / 100) as usize));
            original -= 100 * (original / 100);
        }  if original >= 90 {
            roman.push_str("XC");
            original -= 90;
        }  if original >= 50 {
            roman.push_str("L");
            original -= 50;
        }  if original >= 40 {
            roman.push_str("XL");
            original -= 40;
        }  if original >= 10 {
            roman.push_str(&"X".repeat((original / 10) as usize));
            original -= 10 * (original / 10);
        }  if original >= 9 {
            roman.push_str("IX");
            original -= 9;
        }  if original >= 5 {
            roman.push_str("V");
            original -= 5;
        }if original >= 4 {
            roman.push_str("IV");
            original -= 4;
        }  {
            roman.push_str(&"I".repeat(original as usize));
        }
        return roman;
    }
}

fn main(){
    let roman = Solution::int_to_roman(3749);
    println!("{}", roman);
}