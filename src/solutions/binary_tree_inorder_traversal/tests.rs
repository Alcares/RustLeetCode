use std::cell::RefCell;
use std::rc::Rc;

#[cfg(test)]
mod tests {
    use std::ptr::null;
    use super::*;
    #[test]
    fn test_inorder_traversal() {
        let root = Some(Rc::new(RefCell::new(TreeNode { val: 1, left: None, right:
        Some(Rc::new(RefCell::new(TreeNode { val: 2, left:
        Some(Rc::new(RefCell::new(TreeNode { val: 3, left: None, right: None }))),
            right: None }))) })));

        assert_eq!(Solution::inorder_traversal(root), vec![1, 3, 2]);
    }
}