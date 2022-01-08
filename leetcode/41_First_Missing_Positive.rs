struct Solution {}

impl Solution {
    pub fn first_missing_positive(mut nums: Vec<i32>) -> i32 {
        let n = nums.len();
        for i in 0..n {
            while  0 < nums[i] && nums[i] <= n as i32 && nums[nums[i] as usize - 1] != nums[i] {
                let temp = nums[i];
                nums[i] = nums[temp as usize - 1];
                nums[temp as usize - 1] = temp;
            }
        }
        for i in 0..n {
            if nums[i] != i as i32 + 1 {
                return i as i32 + 1; 
            }
        }
        n as i32 + 1
    }
}

#[cfg(test)]
mod first_missing_positive_test {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::first_missing_positive(vec![1,2,0]), 3);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::first_missing_positive(vec![3,4,-1,1]), 2);
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::first_missing_positive(vec![7,8,9,11,12]), 1);
    }

    #[test]
    fn test_4() {
        assert_eq!(Solution::first_missing_positive(vec![1]), 2);
    }

    #[test]
    fn test_5() {
        assert_eq!(Solution::first_missing_positive(vec![1,1]), 2);
    }
}