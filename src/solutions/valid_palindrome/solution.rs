pub struct Solution;

impl Solution {
    pub fn is_palindrome(input_string: String) -> bool {
        let mut formatted_string = String::new();

        for c in input_string.chars() {
            if c.is_alphanumeric() {
                formatted_string.push(c.to_lowercase().next().unwrap());
            }
        }

        let halfway_point = formatted_string.len() / 2;
        let is_even = if formatted_string.len() % 2 == 0 { true } else { false };
        let len = formatted_string.len();

        let mut stack = Vec::new();

        for (index, char) in formatted_string.chars().enumerate() {
            if index == halfway_point && !is_even {
                continue
            }
            else if index < halfway_point {
                stack.push(char)
            } else {
                if stack.pop() != Some(char) {
                    return false;
                }
            }
        }

        true
    }
}