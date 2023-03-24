use std::collections::VecDeque;

struct Solution {}

impl Solution {
    pub fn shortest_bridge(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let mut grid = grid;
        let mut stack = vec![];
        let mut queue = VecDeque::new();

        for i in 0..n {
            for j in 0..n {
                if grid[i][j] == 1 {
                    grid[i][j] = 0;
                    stack.push((i, j));
                    while let Some((i, j)) = stack.pop() {
                        queue.push_back((i, j, 0));
                        for (next_i, next_j) in [(1, 0), (0, 1), (-1, 0), (0, -1)].iter()
                            .map(|(di, dj)| (i as i32 + di, j as i32 + dj))
                            .filter(|&(i, j)| i >= 0 && i < n as i32 && j >= 0 && j < n as i32)
                            .map(|(i, j)| (i as usize, j as usize)) 
                        {
                            if grid[next_i][next_j] == 1 {
                                grid[next_i][next_j] = 2;
                                stack.push((next_i, next_j));
                            }                        
                        }
                    }
                    
                    while let Some((i, j, distance)) = queue.pop_front() {
                        for (next_i, next_j) in [(1, 0), (0, 1), (-1, 0), (0, -1)].iter()
                            .map(|(di, dj)| (i as i32 + di, j as i32 + dj))
                            .filter(|&(i, j)| i >= 0 && i < n as i32 && j >= 0 && j < n as i32)
                            .map(|(i, j)| (i as usize, j as usize)) 
                        {
                            if grid[next_i][next_j] == 0 {
                                grid[next_i][next_j] = 2;
                                queue.push_back((next_i, next_j, distance + 1));
                            } else if grid[next_i][next_j] == 1 {
                                return distance;
                            }
                        }

                    }
                }
           }
        }
        unreachable!()
    }
}

#[test]
fn test_1() {
    assert_eq!(1, Solution::shortest_bridge(vec![vec![0,1],vec![1,0]]));
}

#[test]
fn test_2() {
    assert_eq!(2, Solution::shortest_bridge(vec![vec![0,1,0],vec![0,0,0],vec![0,0,1]]));
}

#[test]
fn test_3() {
    assert_eq!(1, Solution::shortest_bridge(vec![vec![1,1,1,1,1],vec![1,0,0,0,1],vec![1,0,1,0,1],vec![1,0,0,0,1],vec![1,1,1,1,1]]));
}
