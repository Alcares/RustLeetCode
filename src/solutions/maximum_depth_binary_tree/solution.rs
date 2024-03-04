use crate::utils::trees::*;
type Node = Option<Rc<RefCell<TreeNode>>>;

struct Solution;

impl Solution {
    pub fn max_depth(root: Node) -> i32 {

        let mut deepest_seen = 0;
        let mut stack = vec![(root, 0)];

        while let Some((node, mut depth)) = stack.pop() {
            if let Some(node) = node {
                let node = node.borrow();
                depth += 1;
                deepest_seen = deepest_seen.max(depth);

                stack.push((node.left.clone(), depth));
                stack.push((node.right.clone(), depth));
            }
        }
        deepest_seen
    }
}