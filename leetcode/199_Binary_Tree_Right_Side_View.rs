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
    pub fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        fn rec(node: &Option<Rc<RefCell<TreeNode>>>, level: usize, res: &mut Vec<i32>) {
            if let Some(node) = node.as_ref(){ 
                if res.len() == level {
                    res.push(node.borrow().val);
                }
                rec(&node.borrow().right, level + 1, res);
                rec(&node.borrow().left, level + 1, res);
            }
        }
        let mut res = vec![];
        rec(&root, 0, &mut res);
        res
    }
}
