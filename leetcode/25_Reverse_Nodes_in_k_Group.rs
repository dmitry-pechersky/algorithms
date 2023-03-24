
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

struct Solution { }

impl Solution {
    pub fn reverse_k_group(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        let n = 0;
        let mut current = head;
        let mut res = None;
        let mut res_current = &mut res;
        let mut stack = Vec::with_capacity(k as usize);
        while current.is_some() {
            let mut node = current.take().unwrap();
            current = node.next.take();
            stack.push(node);
            if k as usize == stack.len() {
                while let Some(node) = stack.pop() {
                    res_current.replace(node);
                    res_current = &mut res_current.as_mut().unwrap().next;
                }
            }
        }
        let mut tail = None;
        while let Some(mut node) = stack.pop() {
            node.next = tail;
            tail = Some(node);
        }
        *res_current = tail;
        res
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

    #[test]
    fn test_1() {
        assert_eq!(
            Solution::reverse_k_group(array_to_list(&[1,2,3,4,5]), 2),
            array_to_list(&[2,1,4,3,5])
        )
    }

    #[test]
    fn test_2() {
        assert_eq!(
            Solution::reverse_k_group(array_to_list(&[1,2,3,4,5]), 3),
            array_to_list(&[3,2,1,4,5])
        )
    }
}