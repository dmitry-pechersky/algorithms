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
    pub fn flatten(root: &mut Option<Rc<RefCell<TreeNode>>>) {
        if let Some(node) = root {
            let mut stack = vec![node.clone()];
            let mut last = Rc::new(RefCell::new(TreeNode::new(0)));
            while let Some(node) = stack.pop() {
                let (left, right) = (node.borrow().left.clone(), node.borrow().right.clone());
                node.borrow_mut().left = None;
                node.borrow_mut().right = None;
                last.borrow_mut().right = Some(node.clone());
                last = node;
                if let Some(right) = right {
                    stack.push(right);
                }
                if let Some(left) = left {
                    stack.push(left);
                }
            }
        }
    }
}

mod flatten_test {
    use super::*;

    #[test]
    fn test_1() {
    }    
}