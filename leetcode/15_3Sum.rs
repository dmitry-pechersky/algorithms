struct Solution { }

impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut nums =  nums;
        nums.sort_unstable();
        let mut res = vec![];
        let n = nums.len();
        let mut i: usize = 0;
        while i + 2 < n && nums[i] <= 0 {
            if i == 0 || nums[i] != nums[i - 1] {
                let (mut j, mut k) = (i + 1, n - 1);
                while j < k && nums[i] + nums[j] <= 0 && nums[k] >= 0 {
                    if j > i + 1 && nums[j] == nums[j - 1] {
                        j += 1;
                    }else if k < n - 1 && nums[k] == nums[k + 1]{
                        k -= 1;
                    }else{
                        let val = nums[i] + nums[j] + nums[k];
                        if val == 0 {
                            res.push(vec![nums[i], nums[j], nums[k]]);
                            j += 1;
                            k -= 1;
                        } else if val > 0 {
                            k -= 1;
                        } else {
                            j += 1;
                        }
                    }
                }    
            }
            i += 1;
        }
        res
    }
}

#[cfg(test)]
mod three_sum_test {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::three_sum(vec![-1,0,1,2,-1,-4]), vec![vec![-1,-1,2], vec![-1,0,1]]);
    }

    #[test]
    fn test_2() {
        let empty: Vec<Vec<i32>> = vec![]; 
        assert_eq!(Solution::three_sum(vec![]), empty);
    }

    #[test]
    fn test_3() {
        let empty: Vec<Vec<i32>> = vec![]; 
        assert_eq!(Solution::three_sum(vec![0]), empty);
    }

    #[test]
    fn test_4() {
        assert_eq!(Solution::three_sum(vec![0,0,0]), vec![[0,0,0]]);
    }

}