struct Solution {}

impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        let (mut prev_with_return, mut prev_without_return) = (0, 0);
        for num in nums {
            let with_return = prev_with_return.max(prev_without_return + num);
            let without_return = prev_with_return.max(prev_without_return);
            prev_with_return = with_return;
            prev_without_return = without_return;
        }
        prev_with_return.max(prev_without_return)
    }
}

#[test]
fn test_1() {
    assert_eq!(4, Solution::rob(vec![1,2,3,1]))
}

#[test]
fn test_2() {
    assert_eq!(12, Solution::rob(vec![2,7,9,3,1]))
}
