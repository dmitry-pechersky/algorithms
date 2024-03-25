use std::collections::VecDeque;

pub struct Solution;

impl Solution {
    pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut max_values = Vec::with_capacity(nums.len());
        let mut deq: VecDeque<(usize, i32)> = VecDeque::with_capacity(k as usize + 1);

        for (i, num) in nums.into_iter().enumerate() {
            while deq.front().is_some() && deq.back().unwrap().1 < num {
                deq.pop_back();
            }
            deq.push_back((i, num));            
            if i >= k as usize - 1 {
                if deq.front().unwrap().0 + k as usize <= i {
                    deq.pop_front();
                }
                max_values.push(deq.front().unwrap().1);
            }
        }

        max_values
    }
}

#[test]
fn test_max_sliding_window() {
    assert_eq!(vec![3,3,5,5,6,7], Solution::max_sliding_window(vec![1,3,-1,-3,5,3,6,7], 3));
    assert_eq!(vec![1], Solution::max_sliding_window(vec![1], 1));
    assert_eq!(vec![7,4], Solution::max_sliding_window(vec![7,2,4], 2));
}