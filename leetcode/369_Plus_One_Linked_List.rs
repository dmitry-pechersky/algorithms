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

struct  Solution {}

impl Solution {
    pub fn plus_one(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        fn rec(node: &mut Option<Box<ListNode>>) -> bool {
            if let Some(node) = node.as_mut() {
                if rec(&mut node.next) {
                    let value =  node.val + 1;
                    node.val = value % 10;
                    value > 9
                } else {
                    false
                }
            } else {
                true
            }
        }        

        let mut head  = head;
        if rec(&mut head) {
            Some(Box::new(ListNode { val: 1, next: head }))
        } else {
            head
        }
    }
}
