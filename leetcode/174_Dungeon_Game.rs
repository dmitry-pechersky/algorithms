use std::collections::BinaryHeap;

pub struct Solution;

impl Solution {
    pub fn calculate_minimum_hp(dungeon: Vec<Vec<i32>>) -> i32 {
        let (n, m) = (dungeon.len(), dungeon[0].len());
        let mut heap = BinaryHeap::from([(dungeon[0][0], (dungeon[0][0]), 0, 0)]);
        let mut dp = vec![vec![(i32::MIN, i32::MIN); m]; n];
        while let Some((min_health, health, i, j)) = heap.pop() {
            if i == n - 1 && j == m - 1 {
                return if min_health <= 0 { 1 - min_health } else { 1 };
            }
            for &(next_i, next_j) in [(i + 1, j), (i, j + 1)].iter() {
                if next_i < n && next_j < m {
                    let next_health = health + dungeon[next_i][next_j];
                    let next_min_health = min_health.min(next_health);
                    if next_health > dp[next_i][next_j].0 || next_min_health > dp[next_i][next_j].1 {
                        heap.push((next_min_health, next_health, next_i, next_j));    
                        if next_health >= dp[next_i][next_j].0 && next_min_health >= dp[next_i][next_j].1 {
                            dp[next_i][next_j] = (next_health, next_min_health);
                        }
                    } 
                }
            }
        }
        unreachable!()
    }
}

#[test]
fn test_calculate_minimum_hp() {
    assert_eq!(7, Solution::calculate_minimum_hp(vec![vec![-2,-3,3],vec![-5,-10,1],vec![10,30,-5]]));
    assert_eq!(1, Solution::calculate_minimum_hp(vec![vec![0]]));
}
