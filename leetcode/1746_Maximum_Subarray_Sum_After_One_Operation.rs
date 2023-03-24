struct Solution {}

impl Solution {
    pub fn max_sum_after_operation(nums: Vec<i32>) -> i32 {
        let (mut without_replace, mut with_replace) = (nums[0], nums[0] * nums[0]);
        let mut max = without_replace.max(with_replace);
        for &num in nums.iter().skip(1) {
            let without_replace_tmp = (without_replace + num).max(num);
            let with_replace_tmp = (with_replace + num).max(without_replace + num * num).max(num * num);
            with_replace = with_replace_tmp;
            without_replace = without_replace_tmp;
            max = max.max(with_replace).max(without_replace);
        }
        max
    }
}

#[test]
fn test_1() {
    assert_eq!(17, Solution::max_sum_after_operation(vec![2,-1,-4,-3]));
    assert_eq!(4, Solution::max_sum_after_operation(vec![1,-1,1,1,-1,-1,1]));
}
