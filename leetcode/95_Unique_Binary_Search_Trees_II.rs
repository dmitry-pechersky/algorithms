// Definition for a binary tree node.
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

fn generate_trees(left: i32, right: i32, dp: &mut Vec<Vec<Option<Vec<Option<Rc<RefCell<TreeNode>>>>>>>) {
    if dp[left as usize][right as usize].is_none() {
        let mut trees = vec![];
        if left > right {
            trees.push(None);
        } else if left == right {
            trees.push(Some(Rc::new(RefCell::new(TreeNode::new(left)))));
        } else {
            for root in left..=right {
                generate_trees(left, root - 1, dp);
                generate_trees(root + 1, right, dp);
                let left_trees = dp[left as usize][root as usize - 1].as_ref().unwrap();
                let right_trees = dp[root as usize + 1][right as usize].as_ref().unwrap();
                for left_tree in left_trees {
                    for right_tree in right_trees {
                        trees.push(Some(Rc::new(RefCell::new(TreeNode { val: root, left: left_tree.clone(), right: right_tree.clone()}))));
                    }
                }
            }    
        }
        dp[left as usize][right as usize] = Some(trees);    
    }
}

pub struct Solution;

impl Solution {
    pub fn generate_trees(n: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        let mut dp = vec![vec![None; n as usize + 2]; n as usize + 2];
        generate_trees(1, n, &mut dp);
        dp[1][n as usize].take().unwrap()
    }
}