#[cfg(test)]
mod tests {
    use crate::solutions::single_number::solution::{self, *};
    #[test]
    fn test_valid_palindrome() {
        let input_1 = vec![4, 1, 2, 1, 2];

        assert_eq!(Solution::single_number(input_1), 4);
    }
}