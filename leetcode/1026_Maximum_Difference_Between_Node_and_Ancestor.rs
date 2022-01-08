use std::rc::Rc;
use std::cell::RefCell;
use std::cmp::{min, max};

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
            val: val,
            left: None,
            right: None,
        }
    }
}

struct Solution {}

impl Solution {
    pub fn max_ancestor_diff(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut diff = 0;
        if let Some(node) = root {
            let mut stack = vec![(node.clone(), node.borrow().val, node.borrow().val)];
            while let Some((node, min_value, max_value)) = stack.pop() {
                let val = node.borrow().val;
                let min_value = min(min_value, val);
                let max_value = max(max_value, val);
                diff = max(diff, (min_value - val).abs());
                diff = max(diff, (max_value - val).abs());
                if let Some(left) = &node.borrow().left {
                    stack.push((left.clone(), min_value, max_value));
                }
                if let Some(right) = &node.borrow().right {
                    stack.push((right.clone(), min_value, max_value));              
                }
            }
        }
        diff
    }
}

mod max_ancestor_diff_test {
    use super::*;

    fn node(val: i32, left: Option<Rc<RefCell<TreeNode>>>, right: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        Some(Rc::new(RefCell::new(TreeNode {val, left, right})))
    }

    #[test]
    fn test_1() {
        assert_eq!(
            Solution::max_ancestor_diff(
                node(1, None, node(2, None, node(0, node(3, None, None), None)))
            ), 
            3
        );
    }
}
