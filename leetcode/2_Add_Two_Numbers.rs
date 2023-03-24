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
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let (mut l1, mut l2) = (l1.as_ref(), l2.as_ref());        
        let mut res = None;
        let mut current = &mut res;
        let mut transfer = 0;
        while l1.is_some() && l2.is_some() {
            let value = l1.unwrap().val + l2.unwrap().val + transfer;
            transfer = value / 10;
            current.replace(Box::new(ListNode::new(value % 10)));
            current = &mut current.as_mut().unwrap().next;            
            l1 = l1.unwrap().next.as_ref();
            l2 = l2.unwrap().next.as_ref();
        }
        while l1.is_some() {
            let value = l1.unwrap().val + transfer;
            transfer = value / 10;
            current.replace(Box::new(ListNode::new(value % 10)));
            current = &mut current.as_mut().unwrap().next;            
            l1 = l1.unwrap().next.as_ref();
        }
        while l2.is_some() {
            let value = l2.unwrap().val + transfer;
            transfer = value / 10;
            current.replace(Box::new(ListNode::new(value % 10)));
            current = &mut current.as_mut().unwrap().next;            
            l2 = l2.unwrap().next.as_ref();
        }
        if transfer > 0 {
            current.replace(Box::new(ListNode::new(transfer)));
        }
        res
    }
}
