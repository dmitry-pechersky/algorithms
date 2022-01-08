struct Solution {}

impl Solution {
    pub fn can_partition(nums: Vec<i32>) -> bool {
        let total_sum: i32 = nums.iter().sum();
        if total_sum % 2 != 0 {
            return false
        }
        let target = (total_sum / 2) as usize;
        let mut dp = vec![false; target];
        dp[0] = true;
        for num in nums {
            let num = num as usize;
            for sum in (0..target).rev() {
                if  dp[sum] {
                    if  sum + num < target {
                        dp[sum + num] = true;
                    } else if sum + num == target {
                        return true;
                    }
                } 
            }    
        }
        false
    }
}

mod test_can_partition {
    use super::*;

    #[test]
    fn test_1() {
        assert!(Solution::can_partition(vec![1,5,11,5]));
    }

    #[test]
    fn test_2() {
        assert!(! Solution::can_partition(vec![1,2,3,5]));
    }

    #[test]
    fn test_3() {
        assert!(Solution::can_partition(vec![1,1]));
    }

}
