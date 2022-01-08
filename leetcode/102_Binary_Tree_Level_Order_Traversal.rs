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
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut order: Vec<Vec<i32>> = vec![];
        if let Some(r) = root {
            let mut stack = vec![(r, 0)];
            while let Some((node, level)) = stack.pop() {
                while level >= order.len() {
                    order.push(vec![]);
                }
                order[level].push(node.borrow().val);
                if let Some(right) = &node.borrow().right {
                    stack.push((right.clone(), level + 1));
                }
                if let Some(left) = &node.borrow().left {
                    stack.push((left.clone(), level + 1));
                }
            }
        }
        order
    }
}

#[cfg(test)]
mod level_order_test {
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

        assert_eq!(
            Solution::level_order(Some(n3)),
            vec![vec![3], vec![9,20], vec![15,7]]
        );
    }  

    #[test]
    fn test_2() {
        assert_eq!(Solution::level_order(Some(Rc::new(RefCell::new(TreeNode::new(1))))), vec![vec![1]]);
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::level_order(None), Vec::<Vec<i32>>::new());
    }

}
