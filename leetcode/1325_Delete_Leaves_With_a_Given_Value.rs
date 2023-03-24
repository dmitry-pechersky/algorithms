use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None
        }
    }
}

struct Solution {}

impl Solution {
    pub fn remove_leaf_nodes(root: Option<Rc<RefCell<TreeNode>>>, target: i32) -> Option<Rc<RefCell<TreeNode>>> {
        fn rec(node: &mut Option<Rc<RefCell<TreeNode>>>, target: i32) {
            let remove = if let Some(node) = node.as_mut() {
                let mut node = node.borrow_mut();
                rec(&mut node.left, target);
                rec(&mut node.right, target);
                node.val == target && node.left.is_none() && node.right.is_none()
            } else {
                false
            };
            if remove {
                node.take();
            }
        }

        let mut root = root;
        rec(&mut root, target);
        root
    }
}
