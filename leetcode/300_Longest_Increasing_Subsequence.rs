struct Solution { }

impl Solution {
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        fn bs(vector: &Vec<i32>, mut left: usize, mut right: usize, num: i32) -> usize {
            while left < right {
                let middle = left + (right - left) / 2;
                if vector[middle] == num {
                    return middle;
                } else if vector[middle] < num {
                    left = middle + 1;
                } else {
                    right = middle;
                }
            }
            right
        }

        let mut dp = vec![0; nums.len()];
        let mut l = 0;
        dp[0] = nums[0];
        for &num in nums.iter().skip(1) {
            if num > dp[l] {
                l += 1;
                dp[l] = num;
            } else {
                let idx = bs(&dp, 0, l, num);
                dp[idx] = num;
            }
        }
        l as i32  + 1
    }
}

#[test]
fn test_1() {
    assert_eq!(4, Solution::length_of_lis(vec![10,9,2,5,3,7,101,18]));
}

#[test]
fn test_2() {
    assert_eq!(4, Solution::length_of_lis(vec![0,1,0,3,2,3]));
}

#[test]
fn test_3() {
    assert_eq!(1, Solution::length_of_lis(vec![7,7,7,7,7,7,7]));
}
