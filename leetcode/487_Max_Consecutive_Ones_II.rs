struct Solution {}

impl Solution {
    pub fn find_max_consecutive_ones(nums: Vec<i32>) -> i32 {
        let (mut without_flip, mut with_flip, mut max_cnt) = (0, 0, 0);
        for num in nums {
            if num == 1 {
                with_flip += 1;
                without_flip += 1;
            } else {
                with_flip = without_flip + 1;
                without_flip = 0;
            }
            if with_flip > max_cnt {
                max_cnt = with_flip;
            }
        }
        max_cnt
    }
}

#[test]
fn test_1() {
    assert_eq!(4, Solution::find_max_consecutive_ones(vec![1,0,1,1,0]));
}

#[test]
fn test_2() {
    assert_eq!(4, Solution::find_max_consecutive_ones(vec![1,0,1,1,0,1]));
}
