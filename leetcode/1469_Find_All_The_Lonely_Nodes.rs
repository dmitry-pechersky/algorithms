use std::rc::Rc;
use std::cell::{RefCell, Ref};

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
    pub fn get_lonely_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        fn rec(node: &Rc<RefCell<TreeNode>>, res: &mut Vec<i32>) {
            let node = node.borrow();
            if let Some(left) = node.left.as_ref() {
                if node.right.is_none() {
                    res.push(left.borrow().val);
                }
                rec(left, res);
            }
            if let Some(right) = node.right.as_ref() {
                if node.left.is_none() {
                    res.push(right.borrow().val);
                }
                rec(right, res);
            }
        }
        let mut res = vec![];
        rec(root.as_ref().unwrap(), &mut res);
        res
    }
}
