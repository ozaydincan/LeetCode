struct Solution;

impl Solution {
    pub fn min_bit_flips(start: i32, goal: i32) -> i32 {
        let mut to_be_flipped = start ^ goal;
        let mut count = 0;
        while to_be_flipped != 0 {
            count += to_be_flipped & 1;
            to_be_flipped >>= 1;
        }
        count
    }
}

fn main() {
    let start: i32 = 7;
    let end: i32 = 10;

    let min_flips: i32 = Solution::min_bit_flips(start, end);
    println!("The minimum flips required: {min_flips}");
}

#[cfg(test)]

mod test {
    use crate::Solution;

    #[test]
    fn bit_tests_1() {
        assert_eq!(Solution::min_bit_flips(7, 10), 3);
    }
    #[test]
    fn bit_test_2() {
        assert_eq!(Solution::min_bit_flips(3, 4), 3);
    }
}
