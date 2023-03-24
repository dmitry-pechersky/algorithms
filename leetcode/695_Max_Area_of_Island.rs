struct Solution {}

impl Solution {
    pub fn max_area_of_island(grid: Vec<Vec<i32>>) -> i32 {
        let (n, m) = (grid.len(), grid[0].len());
        let mut grid = grid;
        let mut stack = vec![];
        let mut max_size = 0;
        for i in 0..n {
            for j in 0..m {
                if grid[i][j] == 1 {
                    stack.push((i, j));
                    grid[i][j] = 0;
                    let mut size = 1;
                    while let Some((i, j)) = stack.pop() {
                        for (next_i, next_j) in [(1, 0),(0, 1),(-1, 0),(0, -1)].iter()
                            .map(|(di, dj)| (i as i32 + di, j as i32 + dj))
                            .filter(|&(i, j)| i >= 0 && i < n as i32 && j >= 0 && j < m as i32)
                            .map(|(i, j)| (i as usize, j as usize)) 
                        {
                            if grid[next_i][next_j] == 1 {
                                grid[next_i][next_j] = 0;
                                size += 1;
                                stack.push((next_i, next_j));
                            }

                        }
                    }
                    max_size = max_size.max(size);
                }
            }
        }
        max_size
    }
}

#[test]
fn test_1() {
    assert_eq!(
        6,
        Solution::max_area_of_island(
            [
                [0,0,1,0,0,0,0,1,0,0,0,0,0],
                [0,0,0,0,0,0,0,1,1,1,0,0,0],
                [0,1,1,0,1,0,0,0,0,0,0,0,0],
                [0,1,0,0,1,1,0,0,1,0,1,0,0],
                [0,1,0,0,1,1,0,0,1,1,1,0,0],
                [0,0,0,0,0,0,0,0,0,0,1,0,0],
                [0,0,0,0,0,0,0,1,1,1,0,0,0],
                [0,0,0,0,0,0,0,1,1,0,0,0,0]
            ].iter().map(|array| array.to_vec()).collect()
        )
    );
}

#[test]
fn test_2() {
    assert_eq!(
        0,
        Solution::max_area_of_island(
            [
                [0,0,0,0,0,0,0,0]
            ].iter().map(|array| array.to_vec()).collect()
        )
    );
}