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

impl Solution {
    pub fn build_tree(inorder: Vec<i32>, postorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        None
    }
}

mod is_robot_bounded_test {
    use super::*;

    #[test]
    fn test_1() {
        let root1 = Rc::new(RefCell::new(TreeNode::new(
            2, 
            Some(Rc::new(RefCell::new(TreeNode::new(1, None, None)))), 
            Some(Rc::new(RefCell::new(TreeNode::new(4, None, None))))))
        );
        let root2 = Rc::new(RefCell::new(TreeNode::new(
            1, 
            Some(Rc::new(RefCell::new(TreeNode::new(0, None, None)))), 
            Some(Rc::new(RefCell::new(TreeNode::new(3, None, None))))))
        );
        assert_eq!(Solution::get_all_elements(Some(root1), Some(root2)), vec![0,1,1,2,3,4]);
    }  
}
