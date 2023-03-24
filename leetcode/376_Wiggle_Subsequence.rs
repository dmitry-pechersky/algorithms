struct Solution {}

impl Solution {
    pub fn wiggle_max_length(nums: Vec<i32>) -> i32 {
        let (n, mut cnt) = (nums.len(), 1);

        let mut i = 1;
        while i < n && nums[i] == nums[0] {
            i += 1;
        }
        if i < n {
            let mut increasing = nums[0] < nums[i];
            let mut last = nums[i];
            cnt += 1;
            for &num in &nums[i + 1..n] {
                if increasing && last > num {
                    increasing = false;
                    cnt +=1;
                } else if !increasing && last < num {
                    increasing = true;
                    cnt += 1;
                }
                last = num;
            }
    
        }
        cnt
    }
}

#[test]
fn test_0() {
    assert_eq!(1, Solution::wiggle_max_length(vec![0,0]));
    assert_eq!(2, Solution::wiggle_max_length(vec![1,2,3]));
    assert_eq!(2, Solution::wiggle_max_length(vec![3,2,1]));
}


#[test]
fn test_1() {
    assert_eq!(6, Solution::wiggle_max_length(vec![1,7,4,9,2,5]));
}

#[test]
fn test_2() {
    assert_eq!(7, Solution::wiggle_max_length(vec![1,17,5,10,13,15,10,5,16,8]));
}

#[test]
fn test_3() {
    assert_eq!(2, Solution::wiggle_max_length(vec![1,2,3,4,5,6,7,8,9]));
}