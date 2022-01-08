struct Solution {}

impl Solution {
    pub fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {
        if obstacle_grid[0][0] == 1 {
            return 0;
        }
        let mut grid = [[0u32; 100]; 100];
        let (m, n) = (obstacle_grid.len(), obstacle_grid[0].len());
        for i in 0..m {
            if obstacle_grid[i][0] == 1 {
                break;
            }
            grid[i][0] = 1;
        }
        for i in 1..n {
            if obstacle_grid[0][i] == 1 {
                break;
            }
            grid[0][i] = 1;
        }
        for i in 1..m {
            for j in 1..n {
                if obstacle_grid[i][j] == 0 {
                    grid[i][j] = grid[i - 1][j] + grid[i][j - 1];
                }
            }
        }
        grid[m - 1][n - 1] as i32
    }
}

#[cfg(test)]
mod unique_paths_with_obstacles_test {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::unique_paths_with_obstacles(vec![vec![0,0,0], vec![0,1,0], vec![0,0,0]]), 2);
    }    

    #[test]
    fn test_2() {
        assert_eq!(Solution::unique_paths_with_obstacles(vec![vec![0,1], vec![0,0]]), 1);
    }    
}
