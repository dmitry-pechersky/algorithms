struct Solution {}

impl Solution {
    fn _rec(n: usize, nums: &mut Vec<i32>, i: usize, combinations: &mut Vec<Vec<i32>>) {
        if  i >= n {
            combinations.push(nums.clone())
        } else {
            Self::_rec(n, nums, i + 1, combinations);
            for j in (0..i).rev() {
                nums.swap(i, j);
                Self::_rec(n, nums, i + 1, combinations);
                nums.swap(i, j);
            }
        }
    }

    pub fn permute(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut combinations: Vec<Vec<i32>> = vec![];
        Self::_rec(nums.len(), &mut nums, 1, &mut combinations);
        combinations
    }
}

#[cfg(test)]
mod permute_test {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::permute(vec![1,2,3]), vec![vec![1,2,3], vec![1,3,2], vec![2,1,3], vec![2,3,1], vec![3,1,2], vec![3,2,1]]);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::permute(vec![0,1]), vec![vec![0,1], vec![1,0]]);
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::permute(vec![1]), vec![vec![1]]);
    }
}