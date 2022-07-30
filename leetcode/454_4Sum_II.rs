use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn four_sum_count(nums1: Vec<i32>, nums2: Vec<i32>, nums3: Vec<i32>, nums4: Vec<i32>) -> i32 {
        let mut dic = HashMap::new();
        let mut cnt = 0;
        for a in &nums1 {
            for b in &nums2 {
                let value = 1 + match dic.get(&(a + b)) {
                    Some(value) =>  *value,
                    None => 0,
                };
                dic.insert(a + b, value);
            }
        }
        for a in &nums3 {
            for b in &nums4 {
                if let Some(value) = dic.get(&(- a - b)) {
                    cnt += value;
                }
            }
        }
        cnt
    }
}

mod four_sum_count_test {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::four_sum_count(vec![1,2], vec![-2,-1], vec![-1,2], vec![0,2]), 2);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::four_sum_count(vec![0], vec![0], vec![0], vec![0]), 1);
    }
}