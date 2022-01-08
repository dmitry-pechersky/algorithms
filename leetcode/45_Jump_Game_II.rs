struct Solution {}

impl Solution {
    pub fn jump(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let (mut jump_max, mut next_jump_max, mut jump_cnt) = (0, 0, 0);
        for i in 0..n - 1 {
            if i > jump_max {
                jump_max = next_jump_max;
                jump_cnt += 1;
            }
            if next_jump_max < i + nums[i] as usize {
                next_jump_max = i + nums[i] as usize ;
            }
            if next_jump_max >= n - 1 {
                jump_cnt += 1;
                break;
            }
        }
        jump_cnt as i32
    }
}

#[cfg(test)]
mod jump_test {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::jump(vec![2,3,1,1,4]), 2);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::jump(vec![2,3,0,1,4]), 2);
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::jump(vec![0]), 0);
    }

    #[test]
    fn test_4() {
        assert_eq!(Solution::jump(vec![2,1]), 1);
    }
}