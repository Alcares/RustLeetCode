use crate::utils::trees::*;
struct Solution;

impl Solution {
    pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let (mut current_value, mut next) = (head.clone().unwrap().val, head.unwrap().next);
        while next.is_some() {
            let next_value = next.as_ref().unwrap().val;
            if current_value ==  next_value {

            }
            current_value = next_value;
            next = next.unwrap().next;
        }

        Some(Box::new(ListNode { val: 1, next: None }))
    }
}