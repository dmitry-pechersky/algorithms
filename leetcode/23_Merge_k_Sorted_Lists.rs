use std::collections::BinaryHeap;

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

struct SortingItem<K, T> {
    key: K,
    data: T
}

impl<K, T> SortingItem<K, T> {
    fn new(key: K, data: T) -> Self {
        Self { key, data }
    }
}

impl<K: PartialEq, T> PartialEq for SortingItem<K, T> {
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key
    }
}

impl<K: PartialOrd, T> PartialOrd for SortingItem<K, T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        other.key.partial_cmp(&self.key)
    }    
}

impl<K: Eq, T> Eq for SortingItem<K, T> { }


impl<K: Ord, T> Ord for SortingItem<K, T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.key.cmp(&self.key)
    }
}

struct Solution {}

impl Solution {
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        let mut result = None;
        let mut tail = &mut result;
        let mut heap = BinaryHeap::new();

        for list in lists {
            if let Some(node) = list {
                heap.push(
                    SortingItem::new(
                        node.val,
                        node
                    )
                );
            }
        }

        while let Some(item) = heap.pop() {
            let mut node = item.data;
            if let Some(next_node) = node.next.take() {
                heap.push(
                    SortingItem::new(
                        next_node.val,
                        next_node
                    )
                );
            }
            tail.replace(node);
            tail = &mut tail.as_mut().unwrap().next;
        }
        
        result
    }
}
