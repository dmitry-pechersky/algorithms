struct Solution {}

impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        fn dp(nums: &[i32]) -> i32 {
            let (mut with, mut without) = (0, 0);
            for &num in nums {
                let with_tmp = without + num;
                let without_tmp = without.max(with);
                with = with_tmp;
                without = without_tmp;    
            }            
            with.max(without)
        }

        let n = nums.len();
        if n == 1 {
            nums[0]
        } else {
            dp(&nums[0..n - 1]).max(dp(&nums[1..n]))
        }
    }
}

#[test]
fn test_1() {
    assert_eq!(3, Solution::rob(vec![2,3,2]));
}

#[test]
fn test_2() {
    assert_eq!(4, Solution::rob(vec![1,2,3,1]));
}

#[test]
fn test_3() {
    assert_eq!(3, Solution::rob(vec![2,3,2]));
}