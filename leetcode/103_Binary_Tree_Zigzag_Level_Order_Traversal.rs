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

struct Solution {}

impl Solution {
    pub fn zigzag_level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut res = Vec::<Vec<i32>>::new();
        if let Some(root) = root {
            let mut current_stack = vec![root];
            let mut next_current = Vec::<Rc<RefCell<TreeNode>>>::new();
            let mut level: usize = 0;
            while let Some(node) = current_stack.pop() {
                if current_stack.len() == 0 {
                    
                }
            }
        }
        res
    }
}

#[cfg(test)]
mod zigzag_level_order_test {
    use super::*;

    #[test]
    fn test_1() {
        let n3 = Rc::new(RefCell::new(TreeNode::new(3)));
        let n9 = Rc::new(RefCell::new(TreeNode::new(9)));
        let n20 = Rc::new(RefCell::new(TreeNode::new(20)));
        let n15 = Rc::new(RefCell::new(TreeNode::new(15)));
        let n7 = Rc::new(RefCell::new(TreeNode::new(7)));
        n20.borrow_mut().left = Some(n15);
        n20.borrow_mut().right = Some(n7);
        n3.borrow_mut().left = Some(n9);
        n3.borrow_mut().right = Some(n20);
        assert_eq!(Solution::zigzag_level_order(Some(n3)), vec![vec![3], vec![20,9], vec![15,7]]);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::zigzag_level_order(Some(Rc::new(RefCell::new(TreeNode::new(1))))), vec![vec![1]]);
    }    

    #[test]
    fn test_3() {
        assert_eq!(Solution::zigzag_level_order(None), Vec::<Vec<i32>>::new());
    }    
}