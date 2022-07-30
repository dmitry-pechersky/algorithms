use std::cell::RefCell;
use std::rc::Rc;

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
            right
        }
    }
}

struct BSTIterator {
    stack: Vec<(Rc<RefCell<TreeNode>>, bool)>,
}

impl BSTIterator {

    fn new(root: Option<Rc<RefCell<TreeNode>>>) -> Self {
        if let Some(node) = root {
            BSTIterator { stack: vec![(node, false)] }
        } else {
            BSTIterator { stack: vec![] }
        }
    }
    
    fn next(&mut self) -> i32 {
        while let Some((node, expanded)) = self.stack.pop() {
            if expanded {
                return node.borrow().val;
            } else {
                if let Some(right) = &node.borrow().right {
                    self.stack.push((right.clone(), false));
                }
                self.stack.push((node.clone(), true));
                if let Some(left) = &node.borrow().left {
                    self.stack.push((left.clone(), false));
                }
            }
        }
        0
    }
    
    fn has_next(&self) -> bool {
        ! self.stack.is_empty()
    }
}

#[cfg(test)]
mod word_dictionary_test {
    use super::*;

    #[test]
    fn test_1() {
        let root = Rc::new(RefCell::new(TreeNode::new(
            7, 
            Some(Rc::new(RefCell::new(TreeNode::new(
                3,
                None,
                None,

            )))),
            Some(Rc::new(RefCell::new(TreeNode::new(
                15,
                Some(Rc::new(RefCell::new(TreeNode::new(
                    9,
                    None,
                    None
                )))),
                Some(Rc::new(RefCell::new(TreeNode::new(
                    20,
                    None,
                    None
                ))))                
            ))))
        )));
        let mut  itr = BSTIterator::new(Some(root));
        assert_eq!(itr.next(), 3);
        assert_eq!(itr.next(), 7);
        assert!(itr.has_next());
        assert_eq!(itr.next(), 9);
        assert!(itr.has_next());
        assert_eq!(itr.next(), 15);
        assert!(itr.has_next());
        assert_eq!(itr.next(), 20);
        assert!(!itr.has_next());
    }
}
