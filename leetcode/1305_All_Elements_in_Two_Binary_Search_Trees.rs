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

use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn get_all_elements(root1: Option<Rc<RefCell<TreeNode>>>, root2: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        fn inorder_rec(root: &Option<Rc<RefCell<TreeNode>>>, inorder: &mut Vec<i32>) {
            if let Some(node) = root {
                inorder_rec(&node.borrow().left, inorder);
                inorder.push(node.borrow().val);
                inorder_rec(&node.borrow().right, inorder);
            }
        }

        fn merge(array1: Vec<i32>, array2: Vec<i32>) -> Vec<i32> {
            let mut array = vec![];
            let (n1, n2) = (array1.len(), array2.len());
            let (mut i1, mut i2) = (0, 0);
            while i1 < n1 || i2 < n2 {
                if i1 < n1 && i2 < n2 {
                    if array1[i1] < array2[i2] {
                        array.push(array1[i1]);
                        i1 += 1;
                    } else {
                        array.push(array2[i2]);
                        i2 += 1;
                    }
                } else if i1 < n1 {
                    array.push(array1[i1]);
                    i1 += 1;
                } else {
                    array.push(array2[i2]);
                    i2 += 1;
                }
            }
            array
        }
        let mut inorder1 = vec![];
        let mut inorder2 = vec![];
        inorder_rec(&root1, &mut inorder1);
        inorder_rec(&root2, &mut inorder2);
        merge(inorder1, inorder2)
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

    #[test]
    fn test_2() {
        let root1 = Rc::new(RefCell::new(TreeNode::new(
            1, 
            None, 
            Some(Rc::new(RefCell::new(TreeNode::new(8, None, None))))))
        );
        let root2 = Rc::new(RefCell::new(TreeNode::new(
            8, 
            Some(Rc::new(RefCell::new(TreeNode::new(1, None, None)))), 
            None))
        );
        assert_eq!(Solution::get_all_elements(Some(root1), Some(root2)), vec![1,1,8,8]);
    }    
}