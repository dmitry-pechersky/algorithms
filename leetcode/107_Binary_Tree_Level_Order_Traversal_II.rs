use std::rc::Rc;
use std::cell::RefCell;
use std::collections::VecDeque;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32, left: Option<Rc<RefCell<TreeNode>>>, right: Option<Rc<RefCell<TreeNode>>>) -> Self {
        TreeNode {
            val,
            left,
            right,
        }
    }
}

struct Solution {}

impl Solution {
    pub fn level_order_bottom(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut res: Vec<Vec<i32>> = vec![];
        if let Some(node) = &root {
            let mut queue = VecDeque::<(Rc<RefCell<TreeNode>>, usize)>::new();
            queue.push_back((node.clone(), 0));
            while let Some((node, level)) = queue.pop_front() {
                if level >= res.len() {
                    res.push(vec![])
                }
                res[level].push(node.borrow().val);
                if let Some(left) = &node.borrow().left {
                    queue.push_back((left.clone(), level + 1));
                }
                if let Some(right) = &node.borrow().right {
                    queue.push_back((right.clone(), level + 1));
                }
            }
        }
        res.reverse();
        res
    }
}

mod is_robot_bounded_test {
    use super::*;

    #[test]
    fn test_1() {
        let root = Some(Rc::new(RefCell::new(TreeNode::new(
            3, 
            Some(Rc::new(RefCell::new(TreeNode::new(9, None, None)))), 
            Some(Rc::new(RefCell::new(TreeNode::new(
                20,
                Some(Rc::new(RefCell::new(TreeNode::new(15, None, None)))),
                Some(Rc::new(RefCell::new(TreeNode::new(7, None, None))))
            ))))
        ))));
        assert_eq!(Solution::level_order_bottom(root), vec![vec![15,7], vec![9,20], vec![3]]);
    }

    #[test]
    fn test_2() {
        let root = Some(Rc::new(RefCell::new(TreeNode::new(1, None, None))));
        assert_eq!(Solution::level_order_bottom(root), vec![vec![1]]);
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::level_order_bottom(None), Vec::<Vec<i32>>::new());
    }
}