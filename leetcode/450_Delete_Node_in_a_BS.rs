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

use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn delete_node(root: Option<Rc<RefCell<TreeNode>>>, key: i32) -> Option<Rc<RefCell<TreeNode>>> {

        fn find_least(node: &Rc<RefCell<TreeNode>>) -> i32 {
            if let Some(left) = node.borrow().left.as_ref() {
                find_least(left)
            } else {
                node.borrow().val
            }
        }

        fn delete_node(node: Option<Rc<RefCell<TreeNode>>>, key: i32) -> Option<Rc<RefCell<TreeNode>>> {
            if let Some(node) = node {
                if node.borrow().val < key {
                    let right = delete_node(node.borrow_mut().right.take(), key);
                    node.borrow_mut().right = right;
                    Some(node)
                } else if node.borrow().val > key {
                    let left = delete_node(node.borrow_mut().left.take(), key);
                    node.borrow_mut().left = left;
                    Some(node)
                } else {
                    if node.borrow().left.is_none() {
                        node.borrow_mut().right.take()
                    } else if node.borrow().right.is_none() {
                        node.borrow_mut().left.take()
                    } else {
                        let val = find_least(node.borrow().right.as_ref().unwrap());
                        node.borrow_mut().val = val;
                        let right = node.borrow_mut().right.take();
                        node.borrow_mut().right = delete_node(right, val);
                        Some(node)
                    }
                }
            } else {
                None
            }
        }
        delete_node(root, key)
    }
}
