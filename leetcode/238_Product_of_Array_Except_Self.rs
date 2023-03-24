struct Solution {}

impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut res = vec![0; n];
        let mut mul = 1;
        for i in 0..n {
            res[i] = mul;
            mul = mul * nums[i];
        }
        mul = 1;
        for i in (0..n).rev() {
            res[i] = res[i] * mul;
            mul = mul * nums[i];
        }
        res
    }
}

#[test]
fn test_1() {
    assert_eq!(Solution::product_except_self(vec![1,2,3,4]), vec![24,12,8,6]);
}

#[test]
fn test_2() {
    assert_eq!(Solution::product_except_self(vec![-1,1,0,-3,3]), vec![0,0,9,0,0]);
}
