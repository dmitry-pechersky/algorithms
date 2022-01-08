#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>
}

struct Solution {}

impl Solution {
    pub fn get_decimal_value(mut head: Option<Box<ListNode>>) -> i32 {
        let mut value = 0;
        while let Some(node) = head {
            value = (value << 1) | node.val;
            head = node.next;
        }
        value
    }
}

mod get_decimal_value_test {
    use super::*;

    fn to_list(array: &[i32]) -> Option<Box<ListNode>> {
        let mut head: Option<Box<ListNode>> = None; 
        for num in array.into_iter().rev() {
            head = Some(Box::new(ListNode {next: head, val: *num }));
        }
        head
    }

    #[test]
    fn test_1() {
        assert_eq!(Solution::get_decimal_value(to_list(&[1,0,1])), 5);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::get_decimal_value(to_list(&[0])), 0);
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::get_decimal_value(to_list(&[1])), 1);
    }

    #[test]
    fn test_4() {
        assert_eq!(Solution::get_decimal_value(to_list(&[1,0,0,1,0,0,1,1,1,0,0,0,0,0,0])), 18880);
    }

    #[test]
    fn test_5() {
        assert_eq!(Solution::get_decimal_value(to_list(&[0,0])), 0);
    }
}