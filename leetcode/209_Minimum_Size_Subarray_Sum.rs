struct Solution {}

impl Solution {
    pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut min_cnt = n + 1;
        let mut sum = nums[0];
        let (mut i, mut j) = (0, 0);

        loop {
            if sum >= target {
                min_cnt = min_cnt.min(j - i + 1);
                sum -= nums[i];
                i += 1;
            } else {
                j += 1;
                if j < n {
                    sum += nums[j]
                } else {
                    break;
                }
            }
        }
        if min_cnt <= n { min_cnt as i32 }  else { 0 }
    }
}

#[test]
fn test_1() {     
    assert_eq!(2, Solution::min_sub_array_len(7, vec![2,3,1,2,4,3]));
    assert_eq!(1, Solution::min_sub_array_len(4, vec![1,4,4]));
    assert_eq!(0, Solution::min_sub_array_len(11, vec![1,1,1,1,1,1,1,1]));    
    assert_eq!(5, Solution::min_sub_array_len(15, vec![1,2,3,4,5]));
}