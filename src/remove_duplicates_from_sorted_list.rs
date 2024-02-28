#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}

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
