#[cfg(test)]
mod tests {
    use crate::utils::trees::*;
    use crate::solutions::symmetric_tree::solution::{self, *};
    #[test]
    fn test_inorder_traversal() {
        let tree_one = Some(Rc::new(RefCell::new(TreeNode { val: 1, left: None, right:
        Some(Rc::new(RefCell::new(TreeNode { val: 2, left:
        Some(Rc::new(RefCell::new(TreeNode { val: 3, left: None, right: None }))),
            right: None }))) })));

        let tree_two = Some(Rc::new(RefCell::new(TreeNode { val: 1, left: None, right:
        Some(Rc::new(RefCell::new(TreeNode { val: 2, left:
        Some(Rc::new(RefCell::new(TreeNode { val: 3, left: None, right: None }))),
            right: None }))) })));

        assert_eq!(Solution::is_same_tree(tree_one, tree_two), true);
    }
}