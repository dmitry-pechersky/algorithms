struct Solution {}

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

use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn deepest_leaves_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let (mut max_level, mut max_level_value) = (0, 0);
        if let Some(node) = root {
            let mut stack = vec![(0, node)];
            while let Some((level, node)) = stack.pop() {
                if level > max_level {
                    max_level = level;
                    max_level_value = node.borrow().val;
                } else if level == max_level {
                    max_level_value += node.borrow().val;
                }
                if let Some(child) = node.borrow().left.as_ref() {
                    stack.push((level + 1, child.clone()));
                }
                if let Some(child) = node.borrow().right.as_ref() {
                    stack.push((level + 1, child.clone()));
                }
            }
        }
        max_level_value
    }
}