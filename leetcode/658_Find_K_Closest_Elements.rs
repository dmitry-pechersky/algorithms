use std::ops::Range;

struct Solution { }

impl Solution {
    pub fn find_closest_elements(arr: Vec<i32>, k: i32, x: i32) -> Vec<i32> {
        fn bs(arr: &Vec<i32>, k: usize, x: i32) -> Range<usize> {
            let n = arr.len();
            let (mut left, mut right) = (0, arr.len());
            while right - left > k - 1 {
                let start = (left + right - k) / 2;
                let end = start + k - 1;
                if start > 0 && arr[end] > x && (arr[start - 1] - x).abs() <= (arr[end] - x).abs() {
                    right = end - 1;
                } else if end + 1 < n && arr[start] < x && (arr[end + 1] <= x || (arr[start] - x).abs() > (arr[end + 1] - x).abs()) {
                    left = start + 1;
                } else {
                    return start..end + 1
                }
            }
            left..right + 1  
        }

        arr[bs(&arr, k as usize, x)].to_vec()
    }
}

#[test]
fn test_1() {
    assert_eq!(vec![1,2,3,4], Solution::find_closest_elements(vec![1,2,3,4,5], 4, 3));
    assert_eq!(vec![1,2,3,4], Solution::find_closest_elements(vec![1,2,3,4,5], 4, -1));
    assert_eq!(vec![2,3,4,5], Solution::find_closest_elements(vec![1,2,3,4,5], 4, 100));
    assert_eq!(vec![10], Solution::find_closest_elements(vec![1,1,1,10,10,10], 1, 9));
}
