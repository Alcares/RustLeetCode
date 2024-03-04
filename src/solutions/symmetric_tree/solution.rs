use crate::utils::trees::*;
type Node = Option<Rc<RefCell<TreeNode>>>;
struct Solution;

impl Solution {
    pub fn is_symmetric(root: Node) -> bool {
        Self::dfs(&root, &root)
    }

    fn dfs(n1: &Node, n2: &Node) -> bool {
        match (n1, n2) {
            (Some(n1), Some(n2)) => {
                n1.borrow().val == n2.borrow().val &&
                    Self::dfs(&n1.borrow().left, &n2.borrow().right) &&
                    Self::dfs(&n1.borrow().right, &n2.borrow().left)
            },
            (None, None) => true,
            (_, _) => false,
        }
    }
}