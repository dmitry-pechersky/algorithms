use std::collections::VecDeque;

struct Solution {}

impl Solution {
    pub fn max_distance(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let mut grid = grid;
        let mut queue = VecDeque::new();
        let mut max_distance = 0;
        for i in 0..n {
            for j in 0..n {
                if grid[i][j] == 1 {
                    queue.push_back((i, j, 0));
                }
            }
        }
        while let Some((i, j, distance)) = queue.pop_front() {
            max_distance = max_distance.max(distance);
            for (next_i, next_j) in [(1,0),(0,1),(-1,0),(0,-1)].iter()
                .map(|(di, dj)| (i as i32 + di, j as i32 + dj))
                .filter(|&(i, j)| i >= 0 && i < n as i32 && j >= 0 && j < n as i32)
                .map(|(i, j)| (i as usize, j as usize))
            {
                if grid[next_i][next_j] == 0 {
                    grid[next_i][next_j] = 1;
                    queue.push_back((next_i, next_j, distance + 1));
                }
            }
        }
        if max_distance == 0 { -1 } else { max_distance }
    }
}

#[test]
fn test_1() {
    assert_eq!(
        2,
        Solution::max_distance(
            vec![
                vec![1, 0, 1],
                vec![0, 0, 0],
                vec![1, 0, 1]
            ]
        )
    );
}

#[test]
fn test_2() {
    assert_eq!(
        4,
        Solution::max_distance(
            vec![
                vec![1, 0, 0],
                vec![0, 0, 0],
                vec![0, 0, 0]
            ]
        )
    );
}