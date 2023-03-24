struct Solution {}

impl Solution {
    pub fn max_uncrossed_lines(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let (n1, n2) = (nums1.len(), nums2.len());
        let mut dp_1 = vec![0;n1];
        let mut dp_2 = vec![0;n1];
        for i in 0..n2 {
            dp_2[0] = if nums1[0] == nums2[i] { 1 } else { dp_1[0] }; 
            for j in 1..n1 {
                dp_2[j] = dp_1[j].max(dp_2[j - 1]).max(dp_1[j - 1] + if nums1[j] == nums2[i] { 1 } else { 0 });
            }
            let tmp = dp_2;
            dp_2 = dp_1;
            dp_1 = tmp;
        } 

        dp_1[n1 - 1]
    }
}

#[test]
fn test_1() {
    assert_eq!(2, Solution::max_uncrossed_lines(vec![1,4,2], vec![1,2,4]));
    assert_eq!(3, Solution::max_uncrossed_lines(vec![2,5,1,2,5], vec![10,5,2,1,5,2]));
    assert_eq!(2, Solution::max_uncrossed_lines(vec![1,3,7,1,7,5], vec![1,9,2,5,1]));
}
