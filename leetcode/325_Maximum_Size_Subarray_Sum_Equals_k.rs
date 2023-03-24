use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn max_sub_array_len(nums: Vec<i32>, k: i32) -> i32 {
        let mut dic = HashMap::from([(0, -1)]);
        let mut sum = 0;
        let mut max_len = 0;
        for i in 0..nums.len() as i32 {
            sum += nums[i as usize];
            if ! dic.contains_key(&sum) {
                dic.insert(sum, i as i32);
            }
            if let Some(j) = dic.get(&(sum - k)) {
                max_len = max_len.max(i - j);
            }
        }
        max_len as i32
    }
}

#[test]
fn test_1() {
    assert_eq!(4, Solution::max_sub_array_len(vec![1,-1,5,-2,3], 3));
}

#[test]
fn test_2() {
    assert_eq!(2, Solution::max_sub_array_len(vec![-2,-1,2,1], 1));
}