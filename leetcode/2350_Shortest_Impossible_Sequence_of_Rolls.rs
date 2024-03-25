pub struct Solution;

impl Solution {
    pub fn shortest_sequence(rolls: Vec<i32>, k: i32) -> i32 {
        let mut set = vec![0u32; k as usize + 1];
        let mut len = 1;
        let mut cnt = 0;
        for roll in rolls {
            if set[roll as usize] != len {
                set[roll as usize] = len;
                cnt += 1;
            }
            if cnt == k as usize {
                cnt = 0;
                len += 1;
            }
        }
        len as i32
    }
}

#[test]
fn test_1() {
    assert_eq!(3, Solution::shortest_sequence(vec![4,2,1,2,3,3,2,4,1], 4));
    assert_eq!(2, Solution::shortest_sequence(vec![1,1,2,2], 2));
    assert_eq!(1, Solution::shortest_sequence(vec![1,1,3,2,2,2,3,3], 4));
}

