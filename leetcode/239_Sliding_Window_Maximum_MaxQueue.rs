struct MaxStack {
    vec: Vec<(i32, i32)>
}

impl MaxStack {
    fn push(&mut self, value: i32) {
        let next_max = self.max().map_or(value, |max| max.max(value));
        self.vec.push((value, next_max));
    }

    fn max(&self) -> Option<i32> {
        self.vec.last().map(|(_, max)| *max)
    }

    fn pop(&mut self) -> Option<i32> {
        self.vec.pop().map(|(value, _)| value)
    }

    fn is_empty(&self) -> bool {
        self.vec.is_empty()
    }
}

struct MaxQueue {
    stack1: MaxStack,
    stack2: MaxStack
}

impl  MaxQueue {
    fn with_capacity(capacity: usize) -> Self {
        MaxQueue { stack1: MaxStack { vec: Vec::with_capacity(capacity)}, stack2: MaxStack { vec: Vec::with_capacity(capacity)} }
    }

    fn push_back(&mut self, value: i32) {
        self.stack1.push(value);
    }

    fn pop_front(&mut self) -> Option<i32> {
        if self.stack2.is_empty() {
            while let Some(value) = self.stack1.pop() {
                self.stack2.push(value);
            }
        }    
        self.stack2.pop()
    }

    fn max(&self) -> Option<i32> {
        let (max1, max2) = (self.stack1.max(), self.stack2.max());
        if max1.is_none() {
            max2
        } else if max2.is_none() {
            max1
        } else {
            max2.max(max1)
        }
    }
}

pub struct Solution;

impl Solution {
    pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut max_values = vec![];
        let mut queue = MaxQueue::with_capacity(k as usize);
        for &num in &nums[0..k as usize - 1] {
            queue.push_back(num);
        }
        for &num in &nums[k as usize - 1..] {
            queue.push_back(num);
            max_values.push(queue.max().unwrap());
            queue.pop_front();
        }
        max_values
    }
}

#[test]
fn test_max_sliding_window() {
    assert_eq!(vec![3,3,5,5,6,7], Solution::max_sliding_window(vec![1,3,-1,-3,5,3,6,7], 3));
    assert_eq!(vec![1], Solution::max_sliding_window(vec![1], 1));
}