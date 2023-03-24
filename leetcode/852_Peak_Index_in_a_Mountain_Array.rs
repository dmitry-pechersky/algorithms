struct Solution {}

impl Solution {
    pub fn peak_index_in_mountain_array(arr: Vec<i32>) -> i32 {
        let (mut left, mut right) = (1, arr.len() - 2);
        while left <= right {
            let middle = left + (right - left) / 2;
            if arr[middle] > arr[middle - 1] && arr[middle] > arr[middle + 1] {
                return middle as i32;
            }
            if arr[middle] < arr[middle + 1] {
                left = middle + 1;
            }
            if  arr[middle] < arr[middle - 1] {
                right = middle - 1;
            }
        }
        unreachable!()
    }
}

#[test]
fn test_1() {
    assert_eq!(1, Solution::peak_index_in_mountain_array(vec![0,1,0]));
}

#[test]
fn test_2() {
    assert_eq!(1, Solution::peak_index_in_mountain_array(vec![0,2,1,0]));
}

#[test]
fn test_3() {
    assert_eq!(1, Solution::peak_index_in_mountain_array(vec![0,10,5,2]));
}

#[test]
fn test_4() {
    assert_eq!(1, Solution::peak_index_in_mountain_array(vec![3,5,3,2,0]));
}
