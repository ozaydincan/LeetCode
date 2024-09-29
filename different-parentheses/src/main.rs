struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn diff_ways_to_compute(expression: String) -> Vec<i32> {
        let mut memo = HashMap::new();
        Self::compute(&expression, &mut memo)
    }

    fn compute(expr: &str, memo: &mut HashMap<String, Vec<i32>>) -> Vec<i32> {
        if let Some(result) = memo.get(expr) {
            return result.clone();
        }

        let mut result = Vec::new();
        let chars: Vec<char> = expr.chars().collect();

        for (i, &c) in chars.iter().enumerate() {
            if c == '+' || c == '-' || c == '*' {
                let left_expr = &expr[..i];
                let right_expr = &expr[i + 1..];

                let left_results = Self::compute(left_expr, memo);
                let right_results = Self::compute(right_expr, memo);

                for &left in &left_results {
                    for &right in &right_results {
                        let value = match c {
                            '+' => left + right,
                            '-' => left - right,
                            '*' => left * right,
                            _ => panic!("Unexpected operator: {}", c),
                        };
                        result.push(value);
                    }
                }
            }
        }

        if result.is_empty() {
            result.push(expr.parse().unwrap());
        }

        memo.insert(expr.to_string(), result.clone());
        result
    }
}

fn main() {
    println!("Hello, world!");
}
