use std::{rc::Rc, cell::RefCell};

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
    pub fn rob(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn rec(node: &Option<Rc<RefCell<TreeNode>>>) -> (i32, i32) {
            if let Some(node) = node.as_ref() {
                let node = node.borrow();
                let (left_with, left_without) = rec(&node.left);
                let (right_with, right_without) = rec(&node.right);
                (
                    node.val + left_without + right_without,
                    left_with.max(left_without) + right_with.max(right_without)
                )
            } else {
                (0, 0)
            }
        }        
        let (with, without) = rec(&root);
        with.max(without)
    }
}
