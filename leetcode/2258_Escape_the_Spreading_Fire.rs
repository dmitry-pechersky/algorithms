use std::collections::{VecDeque, BinaryHeap};

const GRASS: i32 = 0;
const FIRE: i32 = 1;

pub struct  Solution;

impl Solution {
    pub fn maximum_minutes(grid: Vec<Vec<i32>>) -> i32 {
        let (n, m) = (grid.len(), grid[0].len());

        let fire_time = Self::bfs_fire_time(&grid);
        let mut heap = BinaryHeap::from([(fire_time[n - 1][m - 1], n - 1, m - 1)]);
        let mut times = vec![vec![-1; m]; n];

        while let Some((time, i, j)) = heap.pop() {
            for (ni, nj) in [(1, 0), (0, 1), (-1, 0), (0, -1)].iter()
                .map(|(di, dj)| (i as isize + di, j as isize + dj))
                .filter(|(ni, nj)| *ni >= 0 && *ni < n as isize && *nj >= 0 && *nj < m as isize)
                .map(|(ni, nj)| (ni as usize, nj as usize))
                .filter(|(ni, nj)| grid[*ni][*nj] == GRASS)
            {
                let next_time = (time - 1).min(fire_time[ni][nj] - 1);
                if next_time > times[ni][nj] {
                    times[ni][nj] = next_time;
                    heap.push((next_time, ni, nj));
                }
            }
        }
        if times[0][0] > 1000000000 { 1000000000 } else { times[0][0] }
    }

    fn bfs_fire_time(grid: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let (n, m) = (grid.len(), grid[0].len());

        let mut fire_times = vec![vec![i32::MAX; m as usize]; n as usize];
        let mut queue = VecDeque::new();

        for i in 0..n {
            for j in 0..m {
                if grid[i][j] == FIRE {
                    queue.push_back((i, j));
                    fire_times[i][j] = 0;
                }
            }
        }

        while let Some((i, j)) = queue.pop_front() {
            let time = fire_times[i][j];
            let (i, j) = (i as isize, j as isize);
            for (ni, nj) in [(i + 1, j), (i, j + 1), (i - 1, j), (i, j - 1)].iter() 
                .filter(|(ni, nj)| *ni >= 0 && *ni < n as isize && *nj >= 0 && *nj < m as isize)
            {
                let (ni, nj) = (*ni as usize, *nj as usize);
                if grid[ni][nj] == GRASS && fire_times[ni][nj] == i32::MAX {
                    fire_times[ni][nj] = time + 1;
                    queue.push_back((ni, nj));
                }
            }
        }
        fire_times
    }
}

#[test]
fn test_maximum_minutes() {
    assert_eq!(Solution::maximum_minutes(vec![vec![0,0,0],vec![2,2,0],vec![1,2,0]]), 1000000000);
    assert_eq!(Solution::maximum_minutes(vec![vec![0,0,0,0],vec![0,1,2,0],vec![0,2,0,0]]), -1);
    assert_eq!(
        Solution::maximum_minutes(
            vec![vec![0,2,0,0,0,0,0],vec![0,0,0,2,2,1,0],vec![0,2,0,0,1,2,0],vec![0,0,2,2,2,0,2],vec![0,0,0,0,0,0,0]]
        ), 
        3
    );
}

