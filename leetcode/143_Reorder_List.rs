#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val
        }
    }
}

struct Solution {}

use  std::collections::VecDeque;

impl Solution {
    pub fn reorder_list(head: &mut Option<Box<ListNode>>) {
        let mut queue = VecDeque::new();
        let mut current = head.take();
        while current.is_some() {
            let mut node = current.unwrap();
            current = node.next.take();
            queue.push_back(node);
        }
        let mut current = head;
        while !queue.is_empty() {
            let mut node = queue.pop_front().unwrap();
            node.next = queue.pop_back();
            current.replace(node);
            current = &mut current.as_mut().unwrap().next;
            if current.is_some() {
                current = &mut current.as_mut().unwrap().next;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*; 
    
    fn array_to_list(array: &[i32]) -> Option<Box<ListNode>> {
        let mut head = None;
        let mut current = &mut head;
        for value in array {
            *current = Some(Box::new(ListNode::new(*value)));
            current = &mut current.as_mut().unwrap().next;
        }
        head
    }

    fn list_to_vec(head: &Option<Box<ListNode>>) -> Vec<i32> {
        let mut current = head;
        let mut res = vec![];
        while let Some(node) = current.as_ref() {
            res.push(node.val);
            current = &node.next;
        }
        res
    }

    #[test]
    fn test_1() {
        let mut list = array_to_list(&[1, 2, 3, 4]);
        Solution::reorder_list(&mut list);
        assert_eq!(list_to_vec(&list), vec![1,4,2,3]);
    }

    #[test]
    fn test_2() {
        let mut list = array_to_list(&[1,2,3,4,5]);
        Solution::reorder_list(&mut list);
        assert_eq!(list_to_vec(&list), vec![1,5,2,4,3]);
    }
}