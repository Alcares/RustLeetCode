#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_stairs() {
        let head = Some(Box::new(ListNode { val: 1, next:
        Some(Box::new(ListNode { val: 3, next:
        Some(Box::new(ListNode {val: 3, next:
        None }))}))}));
        assert_eq!(Solution::delete_duplicates(head), Some(Box::new(ListNode { val: 1, next: None })))
    }
}