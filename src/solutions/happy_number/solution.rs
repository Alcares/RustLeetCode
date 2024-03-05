use std::collections::HashSet;
pub struct Solution;

impl Solution {
    pub fn is_happy(n: i32) -> bool {
        let mut cache: HashSet<i32> = HashSet::new();
        let mut digit_vec: Vec<i32> = n.to_string().chars().map(|c| c.to_digit(10).unwrap() as i32).collect();

        loop {
            let sum: i32 = digit_vec.iter().map(|&x| x * x).sum();
            if sum == 1 {
                return true;
            }
            if cache.contains(&sum) {
                return false;
            }
            cache.insert(sum);
            digit_vec = sum.to_string().chars().map(|c| c.to_digit(10).unwrap() as i32).collect();
        }
    }
}