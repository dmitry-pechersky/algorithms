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
    pub fn find_leaves(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        fn rec(node: &Option<Rc<RefCell<TreeNode>>>, res: &mut Vec<Vec<i32>>) -> usize {
            if let Some(node) = node.as_ref() {
                let node = node.borrow();
                let d = rec(&node.left, res);
                let d =  d.max(rec(&node.right, res));
                if res.len() <= d {
                    res.push(vec![])
                }
                res[d].push(node.val);
                d + 1
            } else {
                0
            }
        }

        let mut res = vec![];
        rec(&root, &mut res);
        res
    }
}
