use std::rc::Rc;
use std::cell::RefCell;

type NodeLink = Rc<RefCell<TreeNode>>;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<NodeLink>,
    pub right: Option<NodeLink>,
}

impl TreeNode {
    fn new_link(val: i32, left: Option<NodeLink>, right: Option<NodeLink>) -> Option<NodeLink> {
        Some(Rc::new(RefCell::new(TreeNode {val, left, right})))
    }
}

pub struct Solution {}

impl Solution {
    pub fn trim_bst(root: Option<Rc<RefCell<TreeNode>>>, low: i32, high: i32) -> Option<Rc<RefCell<TreeNode>>> {
        match root {
            None => None,
            Some(node_link) if node_link.borrow().val > high => {
                Self::trim_bst(node_link.borrow().left.clone(), low, high)
            },
            Some(node_link) if node_link.borrow().val < low => {
                Self::trim_bst(node_link.borrow().right.clone(), low, high)
            },
            Some(node_link) => {
                let left = Self::trim_bst(node_link.borrow().left.clone(), low, high);
                let right = Self::trim_bst(node_link.borrow().right.clone(), low, high);
                node_link.borrow_mut().left = left;
                node_link.borrow_mut().right = right;
                Some(node_link)
            }
        }
    }
}

mod trim_bst {
    use super::*;

    #[test]
    fn test_1() {
        let root = TreeNode::new_link(
            1, 
            TreeNode::new_link(0, None, None), 
            TreeNode::new_link(2, None, None)
        );
        let res = TreeNode::new_link(
            1, 
            None, 
            TreeNode::new_link(2, None, None)
        );
        assert_eq!(Solution::trim_bst(root, 1, 2), res);
    }

    #[test]
    fn test_2() {
        let root = TreeNode::new_link(
            3, 
            TreeNode::new_link(
                0, 
                None, 
                TreeNode::new_link(
                    2, 
                    TreeNode::new_link(1, None, None), 
                    None)), 
            TreeNode::new_link(4, None, None)
        );
        let res = TreeNode::new_link(
            3, 
            TreeNode::new_link(
                2, 
                TreeNode::new_link(1, None, None), 
                None), 
            None
        );
        assert_eq!(Solution::trim_bst(root, 1, 3), res);
    }
}