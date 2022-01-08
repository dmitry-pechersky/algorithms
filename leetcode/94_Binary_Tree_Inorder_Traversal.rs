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
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut res = vec![];
        let mut stack: Vec<(Option<Rc<RefCell<TreeNode>>>, bool)> = vec![(root, false)];
        while let Some((node, expanded)) = stack.pop() {
            if let Some(node) = node {
                if expanded {
                    res.push(node.borrow().val);
                } else {
                    stack.push((node.borrow().right.clone(), false));
                    stack.push((Some(node.clone()), true));
                    stack.push((node.borrow().left.clone(), false));
                }    
            }
        }
        res
    }
}

#[cfg(test)]
mod inorder_traversal_test {
    use super::*;

    #[test]
    fn test_1() {
        let n1 = Rc::new(RefCell::new(TreeNode::new(1)));
        let n2 = Rc::new(RefCell::new(TreeNode::new(2)));
        n1.borrow_mut().left = Some(n2);
        assert_eq!(Solution::inorder_traversal(Some(n1)), vec![2, 1]);
    }    

    #[test]
    fn test_2() {
        let n1 = Rc::new(RefCell::new(TreeNode::new(1)));
        let n2 = Rc::new(RefCell::new(TreeNode::new(2)));
        n1.borrow_mut().left = Some(n2);
        assert_eq!(Solution::inorder_traversal(Some(n1)), vec![2, 1]);
    }    

    #[test]
    fn test_3() {
        let n1 = Rc::new(RefCell::new(TreeNode::new(1)));
        let n2 = Rc::new(RefCell::new(TreeNode::new(2)));
        let n3 = Rc::new(RefCell::new(TreeNode::new(3)));
        n2.borrow_mut().left = Some(n3);
        n1.borrow_mut().right = Some(n2);
        assert_eq!(Solution::inorder_traversal(Some(n1)), vec![1, 3, 2]);
    }    
}