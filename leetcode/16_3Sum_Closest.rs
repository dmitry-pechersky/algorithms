struct Solution { }

impl Solution {
    pub fn three_sum_closest(nums: Vec<i32>, target: i32) -> i32 {
        let mut nums = nums;
        nums.sort();
        let n = nums.len();
        let mut closest = nums[0] + nums[1] + nums[2];
        for i in 0..n - 2 {
            let (mut j, mut k) = (i + 1, n - 1);
            while j < k {
                let s = nums[i] + nums[j] + nums[k];
                if s == target {
                    return target;
                } else if s > target {
                    k -= 1;
                } else {
                    j += 1;
                }
                if (s - target).abs() < (closest - target).abs() {
                    closest = s;
                }
            }
        }
        closest
    }
}

#[cfg(test)]
mod three_sum_test {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::three_sum_closest(vec![-1,2,1,-4], 1), 2);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::three_sum_closest(vec![0,0,0], 1), 0);
    }
}