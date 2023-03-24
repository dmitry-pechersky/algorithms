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

struct  Solution {}

impl Solution {
    pub fn kth_smallest(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
        fn rec(node: &Option<Rc<RefCell<TreeNode>>>, k: &mut i32) -> Option<i32> {
            if let Some(node) = node.as_ref() {
                let node = node.borrow();
                if let Some(res) = rec(&node.left, k) {
                    Some(res)
                } else {
                    *k -= 1;
                    if *k == 0 {
                        Some(node.val)
                    } else {
                        rec(&node.right, k)
                    }
                }
            } else {
                None
            }
        }
        rec(&root, &mut k.clone()).unwrap()
    }
}
