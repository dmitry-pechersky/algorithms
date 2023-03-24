struct Solution {}

impl Solution {
    pub fn max_product(nums: Vec<i32>) -> i32 {
        let (mut min, mut max, mut total_max) = (nums[0], nums[0], nums[0]);
        for &num in nums.iter().skip(1) {
            let max_tmp = num.max(max * num).max(min * num);
            let min_tmp = num.min(max * num).min(min * num);
            max = max_tmp;
            min = min_tmp;
            if max > total_max {
                total_max = max;
            }
        }
        total_max
    }
}

#[test]
fn test_1() {
    assert_eq!(6, Solution::max_product(vec![2,3,-2,4]));
}

#[test]
fn test_2() {
    assert_eq!(0, Solution::max_product(vec![-2,0,-1]));
}