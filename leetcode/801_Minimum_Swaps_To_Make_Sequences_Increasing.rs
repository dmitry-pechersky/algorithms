struct Solution {}

impl Solution {
    pub fn min_swap(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let n = nums1.len();
        let (mut dp0, mut dp1) = (0, 1);
        for i in 1..n {
            let (mut cur_dp0, mut cur_dp1) = (n as i32, n as i32);
            if nums1[i - 1] < nums1[i] && nums2[i - 1] < nums2[i] {
                cur_dp0 = dp0;
                cur_dp1 = dp1 + 1;
            }
            if nums1[i - 1] < nums2[i] && nums2[i - 1] < nums1[i] {
                if dp1 < cur_dp0 {
                    cur_dp0 = dp1;
                }
                if dp0 + 1 < cur_dp1 {
                    cur_dp1 = dp0 + 1;
                }
            }
            dp0 = cur_dp0;
            dp1 = cur_dp1;
        }
        if dp0 < dp1 { dp0 } else { dp1 }
    }
}

#[test]
fn test_1() {
    assert_eq!(1, Solution::min_swap(vec![1,3,5,4], vec![1,2,3,7]))
}

#[test]
fn test_2() {
    assert_eq!(1, Solution::min_swap(vec![0,3,5,8,9], vec![2,1,4,6,9]))
}