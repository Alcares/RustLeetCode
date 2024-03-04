use crate::utils::trees::*;

struct Solution;
type OptNode = Option<Rc<RefCell<TreeNode>>>;

impl Solution {
    pub fn inorder_traversal(root: OptNode) -> Vec<i32> {
        let mut result = Vec::new();
        Self::inorder(&root, &mut result);
        result
    }

    fn inorder(node: &OptNode, v: &mut Vec<i32>) {
        if let Some(node) = node {
            let b = node.borrow();
            Self::inorder(&b.left, v);
            v.push(b.val);
            Self::inorder(&b.right, v);
        }
    }
}