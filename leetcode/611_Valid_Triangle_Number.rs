struct Solution {}

impl Solution {
    pub fn triangle_number(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut cnt = 0;
        if n >= 3 {
            let mut nums = nums;
            nums.sort_unstable();
            for i in 0..n - 2 {
                if nums[i] > 0 {
                    let mut k = i + 1;
                    for j in i + 1..n - 1 {
                        let two_sides_sum = nums[i] + nums[j];
                        while k + 1 < n && nums[k + 1] < two_sides_sum {
                            k += 1;
                        }
                        cnt += k - j;
                    }
                }
            }    
        }
        cnt as i32
    }
}

#[test]
fn test_1() {
    assert_eq!(3, Solution::triangle_number(vec![2,2,3,4]));
    assert_eq!(4, Solution::triangle_number(vec![4,2,3,4]));
    assert_eq!(0, Solution::triangle_number(vec![0]));    
    assert_eq!(0, Solution::triangle_number(vec![1,1,3,4]));    
    assert_eq!(0, Solution::triangle_number(vec![7,0,0,0]));    
}