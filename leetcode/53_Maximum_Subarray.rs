struct Solution {}

impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut current_sum = 0;
        let mut max_sum = nums[0];
        for num in nums {
            current_sum += num;
            if current_sum > max_sum {
                max_sum = current_sum;
            }
            if current_sum < 0 {
                current_sum = 0;
            }
        }
        max_sum
    }
}

#[cfg(test)]
mod max_sub_array_test {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::max_sub_array(vec![-2,1,-3,4,-1,2,1,-5,4]), 6);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::max_sub_array(vec![1]), 1);
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::max_sub_array(vec![5,4,-1,7,8]), 23);
    }
}