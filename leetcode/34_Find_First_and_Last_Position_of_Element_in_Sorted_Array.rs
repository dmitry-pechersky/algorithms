struct Solution {}

impl Solution {
    fn _binary_search_left(nums: &Vec<i32>, target: i32, mut left: usize, mut right: usize) -> Option<usize> {
        while left < right {
            let mid: usize = (left + right) / 2;
            if nums[mid] > target {
                right = mid;
            } else if nums[mid] < target {
                left = mid + 1;
            } else {
                right = mid;
            }
        }
        if nums[left] == target { Some(left) } else { None }
    }

    fn _binary_search_right(nums: &Vec<i32>, target: i32, mut left: usize, mut right: usize) -> Option<usize> {
        while left < right {
            let mid: usize = (left + right + 1) / 2;
            if nums[mid] > target {
                right = mid - 1;
            } else if nums[mid] < target {
                left = mid + 1;
            } else {
                left = mid;
            }
        }
        if nums[left] == target { Some(left) } else { None }
    }

    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let not_found = vec![-1, -1];
        match nums.len() {
            0 => not_found,
            n => match Self::_binary_search_left(&nums, target, 0, n - 1) {
                None => not_found,
                Some(left) => match Self::_binary_search_right(&nums, target, left, n - 1) {
                    None => not_found,
                    Some(right) => vec![left as i32, right as i32],
                },
            },
        }
    }
}

#[cfg(test)]
mod search_range_test {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::search_range(vec![5,7,7,8,8,10], 8), vec![3, 4]);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::search_range(vec![5,7,7,8,8,10], 6), vec![-1, -1]);
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::search_range(vec![], 8), vec![-1, -1]);
    }

    #[test]
    fn test_4() {
        assert_eq!(Solution::search_range(vec![2, 2], 1), vec![-1, -1]);
    }
}
