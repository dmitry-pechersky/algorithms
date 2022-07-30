use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

struct Solution {}

impl TreeNode {
    #[inline]
    fn new(val: i32, left: Option<Rc<RefCell<TreeNode>>>, right: Option<Rc<RefCell<TreeNode>>>) -> Self {
        TreeNode {val, left, right}
    }
}

impl Solution {
    pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> Vec<Vec<i32>> {
        fn rec(node: &Option<Rc<RefCell<TreeNode>>>, target_sum: i32, path: &mut Vec<i32>, paths: &mut Vec<Vec<i32>>) {
            if let Some(node) = node {
                let value = node.borrow().val;
                let target_sum = target_sum - value;
                path.push(value);
                if node.borrow().left.is_none() && node.borrow().right.is_none() {
                    if target_sum == 0 {
                        paths.push(path.clone());
                    }
                } else {
                    rec(&node.borrow().left, target_sum, path, paths);
                    rec(&node.borrow().right, target_sum, path, paths);
                }
                path.pop();
            } 
        }
        let mut paths = vec![];
        rec(&root, target_sum, &mut vec![], &mut paths);
        paths 
    }
}

mod path_sum_test {
    use super::*;

    #[test]
    fn test_1() {
        let tree = Rc::new(RefCell::new(TreeNode::new(
            5,
            Some(Rc::new(RefCell::new(TreeNode::new(
                4,
                Some(Rc::new(RefCell::new(TreeNode::new(
                    11,
                    Some(Rc::new(RefCell::new(TreeNode::new(7, None, None)))),
                    Some(Rc::new(RefCell::new(TreeNode::new(2, None, None))))
                )))),
                None
            )))),
            Some(Rc::new(RefCell::new(TreeNode::new(
                8,
                Some(Rc::new(RefCell::new(TreeNode::new(13, None, None)))),
                Some(Rc::new(RefCell::new(TreeNode::new(
                    4,
                    Some(Rc::new(RefCell::new(TreeNode::new(5, None, None)))),
                    Some(Rc::new(RefCell::new(TreeNode::new(1, None, None))))
                ))))
            ))))
        )));
        let mut res = Solution::path_sum(Some(tree), 22);
        res.sort();
        assert_eq!(res, vec![vec![5,4,11,2], vec![5,8,4,5]]);
    }

    #[test]
    fn test_2() {
        let tree = Rc::new(RefCell::new(TreeNode::new(
            1,
            Some(Rc::new(RefCell::new(TreeNode::new(2, None, None)))),
            Some(Rc::new(RefCell::new(TreeNode::new(3, None, None))))
        )));
        assert_eq!(
            Solution::path_sum(Some(tree), 5), 
            Vec::<Vec<i32>>::new()
        );

    }

    #[test]
    fn test_3() {
        let tree = Rc::new(RefCell::new(TreeNode::new(
            1,
            Some(Rc::new(RefCell::new(TreeNode::new(2, None, None)))),
            None
        )));
        assert_eq!(
            Solution::path_sum(Some(tree), 0), 
            Vec::<Vec<i32>>::new()
        );

    }

}