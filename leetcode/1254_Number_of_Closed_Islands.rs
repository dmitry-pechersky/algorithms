struct Solution {}

impl Solution {
    pub fn closed_island(grid: Vec<Vec<i32>>) -> i32 {
        let mut grid = grid;
        let (n, m) = (grid.len(), grid[0].len());
        let mut stack = vec![];
        let mut cnt = 0;
        for i in 0..n {
            for j in 0..m {
                if grid[i][j] == 0 {
                    let mut is_closed = true;
                    stack.push((i, j));
                    while let Some((i,j)) = stack.pop() {
                        if grid[i][j] == 0 {
                            grid[i][j] = 1;
                            if i == 0 || j == 0 || i == n - 1 || j == m - 1 {
                                is_closed = false;
                            }
                            for (next_i, next_j) in [(1,0),(0,1),(-1,0),(0,-1)].iter()
                                .map(|(di, dj)| (i as i32 + di, j as i32 + dj))
                                .filter(|&(i, j)| i >= 0 && i < n as i32 && j >= 0 && j < m as i32)
                                .map(|(i, j)| (i as usize, j as usize)) 
                            {
                                if grid[next_i][next_j] == 0 {
                                    stack.push((next_i, next_j));
                                }
                            }
                        }
                    }
                    if is_closed {
                        cnt += 1;
                    }
                }
            }
        }        
        cnt
    }
}

#[test]
fn test_1() {
    assert_eq!(
        2,
        Solution::closed_island(
            [
                [1,1,1,1,1,1,1,0],
                [1,0,0,0,0,1,1,0],
                [1,0,1,0,1,1,1,0],
                [1,0,0,0,0,1,0,1],
                [1,1,1,1,1,1,1,0]
            ].iter().map(|v| v.to_vec()).collect()
        )
    );
}

#[test]
fn test_2() {
    assert_eq!(
        1,
        Solution::closed_island(
            [
                [0,0,1,0,0],
                [0,1,0,1,0],
                [0,1,1,1,0]
            ].iter().map(|v| v.to_vec()).collect()
        )
    );
}

#[test]
fn test_3() {
    assert_eq!(
        2,
        Solution::closed_island(
            [
                [1,1,1,1,1,1,1],
                [1,0,0,0,0,0,1],
                [1,0,1,1,1,0,1],
                [1,0,1,0,1,0,1],
                [1,0,1,1,1,0,1],
                [1,0,0,0,0,0,1],
                [1,1,1,1,1,1,1]
            ].iter().map(|v| v.to_vec()).collect()
        )
    );
}
