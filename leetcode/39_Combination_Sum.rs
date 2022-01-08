struct Solution {}

impl Solution {
    fn _rec(candidates: &Vec<i32>, target: i32, pos: usize, total: i32, combination: &mut Vec<i32>, combinations: &mut Vec<Vec<i32>>) {
        if total == target {
            combinations.push(combination.clone());
        } else if total + candidates[pos] <= target {
            for i in pos..candidates.len() {
                combination.push(candidates[i]);
                Self::_rec(candidates, target, i, total + candidates[i], combination, combinations);
                combination.pop();
            }
        }
    }

    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut candidates = candidates;
        let mut combination = Vec::<i32>::new();
        let mut combinations = Vec::<Vec<i32>>::new();
        candidates.sort();
        Self::_rec(&candidates, target, 0, 0, &mut combination, &mut combinations);
        combinations
    }
}

#[cfg(test)]
mod combination_sum_test {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::combination_sum(vec![2,3,6,7], 7), vec![vec![2,2,3], vec![7]]);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::combination_sum(vec![2,3,5], 8), vec![vec![2,2,2,2], vec![2,3,3], vec![3,5]]);
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::combination_sum(vec![2], 1), Vec::<Vec<i32>>::new());
    }

    #[test]
    fn test_4() {
        assert_eq!(Solution::combination_sum(vec![1], 1), vec![vec![1]]);
    }

    #[test]
    fn test_5() {
        assert_eq!(Solution::combination_sum(vec![1], 2), vec![vec![1, 1]]);
    }
}