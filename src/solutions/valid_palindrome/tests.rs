#[cfg(test)]
mod tests {
    use crate::solutions::valid_palindrome::solution::{self, *};
    #[test]
    fn test_valid_palindrome() {
        let input_1 = "A man, a plan, a canal: Panama".to_string();
        let input_2 = "race a car".to_string();

        assert_eq!(Solution::is_palindrome(input_1), true);
        assert_eq!(Solution::is_palindrome(input_2), false);
    }
}