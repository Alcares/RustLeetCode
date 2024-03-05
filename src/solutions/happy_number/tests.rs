#[cfg(test)]
mod tests {
    use crate::solutions::happy_number::{self, *};
    use crate::solutions::happy_number::solution::Solution;

    #[test]
    pub fn test_happy_number() {
        assert_eq!(Solution::is_happy(19), true);
        assert_eq!(Solution::is_happy(2), false);
        assert_eq!(Solution::is_happy(7), true);
    }
}