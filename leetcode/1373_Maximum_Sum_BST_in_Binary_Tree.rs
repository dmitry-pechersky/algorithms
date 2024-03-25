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

fn bst_sum(node: &Option<Rc<RefCell<TreeNode>>>, max_sum: &mut i32) -> (bool, i32, Option<i32>, Option<i32>) {
    if let Some(node) = node.as_ref().map(|v| v.borrow()) {
        let (is_left_bst, left_sum, left_min, left_max) = bst_sum(&node.left, max_sum);
        let (is_right_bst, right_sum, right_min, right_max) = bst_sum(&node.right, max_sum);
        if is_left_bst && is_right_bst && (left_max.is_none() || left_max.unwrap() < node.val) && (right_min.is_none() || node.val < right_min.unwrap()) {
            let sum = left_sum + node.val + right_sum;
            *max_sum = sum.max(*max_sum);
            return (true, sum, left_min.or(Some(node.val)), right_max.or(Some(node.val)));
        }
        return (false, 0, None, None);
    }
    (true, 0, None, None)
}

pub struct Solution;

impl Solution {
    pub fn max_sum_bst(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut max_sum = i32::MIN;
        bst_sum(&root, &mut max_sum);
        if max_sum < 0 { 0 } else { max_sum }
    }
}
