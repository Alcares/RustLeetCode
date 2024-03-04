use crate::utils::trees::*;
type OptNode = Option<Rc<RefCell<TreeNode>>>;


pub struct Solution;
impl Solution {
    pub fn is_same_tree(p: OptNode, q: OptNode) -> bool {
        match (p, q) {
            (None, None) => true,
            (None, Some(_)) => false,
            (Some(_), None) => false,
            (Some(ref m), Some(ref n)) => {
                let m = m.borrow();
                let n = n.borrow();
                m.val == n.val && Self::is_same_tree(n.left.clone(), m.left.clone()) && Self::is_same_tree(n.right.clone(), m.right.clone())
            }
        }
    }
}