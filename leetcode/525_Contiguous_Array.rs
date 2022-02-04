use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn find_max_length(nums: Vec<i32>) -> i32 {
        let mut dic = HashMap::new();
        let mut max_length = 0;
        let mut cnt = 0;
        dic.insert(0, 0);
        for i in 0..nums.len() {
            cnt += if nums[i] == 0 { 1 } else { -1 };
            if let Some(j) = dic.get(&cnt) {
                if i - j > max_length {
                    max_length = i - j + 1;
                }
            } else {
                dic.insert(cnt, i + 1);
            }
        }
        max_length as i32
    }
}

mod find_max_length_test {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::find_max_length(vec![0,1]), 2);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::find_max_length(vec![0,1,0]), 2);
    }

}