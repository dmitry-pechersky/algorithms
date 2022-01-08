struct Solution {}

impl Solution {
    fn _binary_search(nums: Vec<i32>, target: i32, mut left: usize, mut right: usize) -> Option<usize> {
        while left < right {
            let mid = (left + right) / 2;
            if nums[mid] < target {
                left = mid + 1;
            } else {
                right = mid;
            }
        }
        if nums[left] == target { Some(left) } else { None }
    }

    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let n = nums.len();
        let (mut left, mut right) = (0, n - 1);
        let mut mid: usize;
        while left < right {
            mid = (left + right) / 2;
            if nums[mid] > nums[right] {
                left = mid + 1;
            } else {
                right = mid;
            }
        }
        let smallest = left;
        if smallest == 0 {
            left = 0;
            right = n - 1;
        } else if target >= nums[0] {
            left = 0;
            right = smallest - 1;
        } else {
            left = smallest;
            right = n - 1;
        }
        match Self::_binary_search(nums, target, left, right) {
            Some(index) => index as i32,
            None => -1,
        }
    }
}

#[cfg(test)]
mod search_test {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::search(vec![4,5,6,7,0,1,2], 0), 4);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::search(vec![4,5,6,7,0,1,2], 3), -1);
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::search(vec![1], 0), -1);
    }    

    #[test]
    fn test_4() {
        assert_eq!(Solution::search(vec![3,1], 3), 0);
    }    
}

#[cfg(test)]
mod binary_search_test {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::_binary_search(vec![1], 0, 0, 0), None);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::_binary_search(vec![1,2,3], 1, 0, 2), Some(0));
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::_binary_search(vec![1,3], 1, 0, 1), Some(0));
    }
}
