struct Solution {}

impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        fn rec(i: usize, nums: &Vec<i32>, combination: &mut Vec<i32>, combinations: &mut Vec<Vec<i32>>) {
            if i >= nums.len() {
                combinations.push(combination.clone());
            } else {
                rec(i + 1, nums, combination, combinations);
                combination.push(nums[i]);
                rec(i + 1, nums, combination, combinations);
                combination.pop();
            }
        }
        let mut combinations: Vec<Vec<i32>> = vec![];
        rec(0, &nums, &mut vec![], &mut combinations);
        combinations       
    }
}

#[cfg(test)]
mod sort_colors_test {
    use super::*;

    #[test]
    fn test_1() {
        let mut result = Solution::subsets(vec![1,2,3]);
        let mut target = vec![vec![], vec![1], vec![2], vec![1,2], vec![3], vec![1,3], vec![2,3], vec![1,2,3]];
        result.sort();
        target.sort();
        assert_eq!(result, target);
    }    

    #[test]
    fn test_2() {
        assert_eq!(Solution::subsets(vec![0]), vec![vec![], vec![0]]);
    }    
}