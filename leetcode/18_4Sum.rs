struct Solution {}

impl Solution {
    pub fn four_sum(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut nums = nums;
        let n = nums.len();
        let mut quadruplets: Vec<Vec<i32>> = vec![];
        if n < 4 {
            return quadruplets;
        }
        nums.sort();
        for i in 0 .. n - 3 {
            if i == 0 || nums[i] > nums[i - 1]{
                for j in i + 1 .. n - 2 {
                    if j == i + 1 || nums[j] > nums[j - 1] {
                        let mut k  = j + 1;
                        let mut l = n - 1;
                        while k < l {
                            let nums_sum = nums[i] + nums[j] + nums[k] + nums[l];
                            if nums_sum == target {
                                if k == j + 1 || nums[k] > nums[k - 1]{
                                    quadruplets.push(vec![nums[i], nums[j], nums[k], nums[l]]);
                                }
                                k += 1;
                                l -= 1;
                            } else if nums_sum > target {
                                l -= 1;
                            } else {
                                k += 1;
                            }
                        } 
                    } 
                }
            }
        }
        quadruplets
    }
}

#[cfg(test)]
mod four_sum_test {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::four_sum(vec![1,0,-1,0,-2,2], 0), vec![vec![-2,-1,1,2], vec![-2,0,0,2], vec![-1,0,0,1]]);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::four_sum(vec![2,2,2,2], 8), vec![vec![2,2,2,2]]);
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::four_sum(vec![2,2,2,2,2,2,2,2,2,2,2], 8), vec![vec![2,2,2,2]]);
    }
}