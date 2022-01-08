#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

use std::rc::Rc;
use std::cell::RefCell;

struct Solution {}

impl Solution {
    pub fn find_tilt(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut tilt = 0;
        fn rec(node: &Option<Rc<RefCell<TreeNode>>>, tilt: &mut i32) -> i32 {
            if let Some(n) = node {
                let right_sum = rec(&n.borrow().right, tilt);                
                let left_sum = rec(&n.borrow().left, tilt);                
                *tilt += (right_sum - left_sum).abs();
                right_sum + left_sum + n.borrow().val
            }
            else {
                0
            }
        }
        rec(&root, &mut tilt);
        tilt
    }
}

mod find_tilt_test {
    use super::*;

    #[test]
    fn test_1() {
        let node2 = Some(Rc::new(RefCell::new(TreeNode {val: 2, left: None, right: None})));
        let node3 = Some(Rc::new(RefCell::new(TreeNode {val: 3, left: None, right: None})));
        let node1 = Some(Rc::new(RefCell::new(TreeNode {val: 1, left: node2, right: node3})));

        assert_eq!(Solution::find_tilt(node1), 1);
    }

    #[test]
    fn test_2() {
        let node3 = Some(Rc::new(RefCell::new(TreeNode {val: 3, left: None, right: None})));
        let node5 = Some(Rc::new(RefCell::new(TreeNode {val: 5, left: None, right: None})));
        let node7 = Some(Rc::new(RefCell::new(TreeNode {val: 7, left: None, right: None})));
        let node2 = Some(Rc::new(RefCell::new(TreeNode {val: 2, left: node3, right: node5})));
        let node9 = Some(Rc::new(RefCell::new(TreeNode {val: 9, left: None, right: node7})));
        let node4 = Some(Rc::new(RefCell::new(TreeNode {val: 4, left: node2, right: node9})));

        assert_eq!(Solution::find_tilt(node4), 15);
    }
}