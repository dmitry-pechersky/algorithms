use std::collections::{HashSet, VecDeque};

struct Solution {}

impl Solution {
    pub fn minimum_moves(grid: Vec<Vec<i32>>) -> i32 {
        fn succesors(grid: &Vec<Vec<i32>>, n: usize, state: (usize, usize, bool)) -> Vec<(usize, usize, bool)> {
            let mut res = vec![];
            let (x, y, horisontal) = state;
            if horisontal {
                if y + 2 < n && grid[x][y + 2] == 0 {
                    res.push((x , y + 1, true));
                }
                if x + 1 < n && y + 1 < n && grid[x + 1][y] == 0 && grid[x + 1][y + 1] == 0 {
                    res.push((x + 1, y, true));
                }
                if x + 1 < n && y + 1  < n && grid[x + 1][y + 1] == 0 && grid[x + 1][y] == 0 {
                    res.push((x, y, false));
                }
            } else {
                if x + 2 < n && grid[x + 2][y] == 0 {
                    res.push((x + 1, y, false));
                }
                if x + 1 < n && y + 1 < n && grid[x][y + 1] == 0 && grid[x + 1][y + 1] == 0 {
                    res.push((x , y + 1, false));
                }
                if x + 1 < n && y + 1 < n && grid[x + 1][y + 1] == 0 && grid[x][y + 1] == 0 {
                    res.push((x, y, true));
                }

            }
            res
        }
        let n = grid.len();
        let state = (0, 0, true);
        let mut queue = VecDeque::from([(0, state)]);
        let mut visited = HashSet::from([state]);
        while let Some((cost, state)) = queue.pop_front() {
            if state == (n - 1, n - 2, true) {
                return cost;
            }
            for next_state in succesors(&grid, n, state).into_iter(){
                if ! visited.contains(&next_state) {
                    visited.insert(next_state);
                    queue.push_back((cost + 1, next_state));
                }
            }
        }
        -1
    }
}

#[test]
fn test_1() {
    let grid = vec![
        vec![0,0,0,0,0,1],
        vec![1,1,0,0,1,0],
        vec![0,0,0,0,1,1],
        vec![0,0,1,0,1,0],
        vec![0,1,1,0,0,0],
        vec![0,1,1,0,0,0]
        ];
    assert_eq!(11, Solution::minimum_moves(grid));
}

#[test]
fn test_2() {
    let grid = vec![
        vec![0,0,1,1,1,1],
        vec![0,0,0,0,1,1],
        vec![1,1,0,0,0,1],
        vec![1,1,1,0,0,1],
        vec![1,1,1,0,0,1],
        vec![1,1,1,0,0,0]
        ];
    assert_eq!(9, Solution::minimum_moves(grid));
}
