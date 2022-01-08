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
use std::collections::HashMap;

impl Solution {
    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        pub fn _rec(preorder: &Vec<i32>, inorder: &Vec<i32>, first: isize, last: isize, inorder_idx_dic: &HashMap<i32, isize>, preorder_idx: &mut usize) -> Option<Rc<RefCell<TreeNode>>> {
            if first > last {
                None
            } else {
                let val = preorder[*preorder_idx];
                let inorder_idx = inorder_idx_dic[&val];
                *preorder_idx += 1;
                Some(Rc::new(RefCell::new(
                    TreeNode {
                        val,
                        left: _rec(preorder, inorder, first, inorder_idx  - 1, inorder_idx_dic, preorder_idx),
                        right: _rec(preorder, inorder, inorder_idx  + 1, last, inorder_idx_dic, preorder_idx)
                    }
                )))
            }
        }
        let mut inorder_idx_dic = HashMap::new();
        for i in 0..inorder.len() {
            inorder_idx_dic.insert(inorder[i], i as isize);
        }
        let mut preorder_idx = 0;
        _rec(&preorder, &inorder, 0, inorder.len() as isize - 1, &inorder_idx_dic, &mut preorder_idx)
    }
}

#[cfg(test)]
mod build_tree_test {
    use super::*;

    fn node(val: i32, left: Option<Rc<RefCell<TreeNode>>>, right: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        Some(Rc::new(RefCell::new(TreeNode {val, left, right})))
    }

    #[test]
    fn test_1() {
        assert_eq!(
            Solution::build_tree(
                vec![3,9,20,15,7], vec![9,3,15,20,7]
            ), 
            node(3, node(9, None, None), node(20, node(15, None, None), node(7, None, None)))
        );
    }    

    #[test]
    fn test_2() {
        assert_eq!(
            Solution::build_tree(vec![-1], vec![-1]), 
            node(-1, None, None)
        );
    }    
}
