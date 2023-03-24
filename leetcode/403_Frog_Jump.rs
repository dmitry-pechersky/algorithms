use std::collections::HashSet;
struct Solution {}

impl Solution {
    pub fn can_cross(stones: Vec<i32>) -> bool {
        let target = *stones.last().unwrap() as i64;
        let heights = stones.into_iter().map(|v| v as i64).collect::<HashSet<_>>();
        let mut visited = HashSet::new();
        let mut stack = vec![(0, 1)];

        while let Some((h, k))  = stack.pop() {
            let next_h = h + k;
            if next_h == target {
                return true;
            }
            if heights.contains(&next_h) && visited.insert((h,k)) {
                for dk in [-1, 0, 1] {
                    stack.push((next_h, k + dk));
                }
            }
        }
        false
    }
}

#[test]
fn test_1() {
    assert!(Solution::can_cross(vec![0,1,3,5,6,8,12,17]));
}

#[test]
fn test_2() {
    assert!(!Solution::can_cross(vec![0,1,2,3,4,8,9,11]));
}
