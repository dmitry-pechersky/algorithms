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

pub struct Solution;

use std::rc::Rc;
use std::cell::RefCell;

fn max_path_sum(root: Option<&Rc<RefCell<TreeNode>>>) -> (i32, i32) {
    if let Some(node) = root {
        let node = node.borrow();
        let (left_path_sum, left_half_path_sum) = max_path_sum(node.left.as_ref());
        let (right_path_sum, right_half_path_sum) = max_path_sum(node.right.as_ref());
        let path_sum = node.val + left_half_path_sum.max(0) + right_half_path_sum.max(0);
        let half_path_sum = node.val + left_half_path_sum.max(right_half_path_sum).max(0);
        (
            left_path_sum.max(right_path_sum).max(path_sum),
            half_path_sum
        )
    } else {
        (i32::MIN, i32::MIN)
    }
}

impl Solution {
    pub fn max_path_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        max_path_sum(root.as_ref()).0
    }
}
