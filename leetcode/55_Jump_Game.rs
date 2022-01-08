struct Solution {}

impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        let n = nums.len();
        let mut max_jump = 0usize;
        for i in 0..n {
            if max_jump < i {
                return false;
            } 
            if i + nums[i] as usize > max_jump {
                max_jump = i + nums[i] as usize;
                if max_jump >= n - 1 {
                    break;
                }
            }
        }
        true
    }
}

#[cfg(test)]
mod can_jump_test {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::can_jump(vec![2,3,1,1,4]), true);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::can_jump(vec![3,2,1,0,4]), false);
    }
}