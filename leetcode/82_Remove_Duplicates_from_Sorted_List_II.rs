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

impl Solution {
    pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut head = head;
        let mut current = &mut head;
        while current.is_some() {
            let cur_value = current.as_ref().unwrap().val;
            if current.as_ref().unwrap().next.is_some() && cur_value == current.as_ref().unwrap().next.as_ref().unwrap().val {
                while current.is_some() && cur_value == current.as_ref().unwrap().val {
                    *current = current.as_mut().unwrap().next.take();
                }
            } else {
                current = &mut current.as_mut().unwrap().next;
            }
        }
        head
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn array_into_list(slice: &[i32]) -> Option<Box<ListNode>> {
        if slice.is_empty() {
            None
        } else {
            let mut node = Box::new(ListNode::new(slice[0]));
            node.next = array_into_list(&slice[1..]);
            Some(node)
        }
    }
    
    #[test]
    fn test_1() {
        assert_eq!(
            Solution::delete_duplicates(array_into_list(&[1,2,3,3,4,4,5])), 
            array_into_list(&[1,2,5])
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            Solution::delete_duplicates(array_into_list(&[1,1,1,2,3])), 
            array_into_list(&[2,3])
        );
    }
}