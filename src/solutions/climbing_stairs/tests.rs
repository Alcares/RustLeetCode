#[cfg(test)]
mod tests {
    use crate::solutions::climbing_stairs::solution::{self, *};
    #[test]
    fn test_stairs() {
        assert_eq!(Solution::climb_stairs(2), 2);
        assert_eq!(Solution::climb_stairs(45), 1836311903);
    }
}