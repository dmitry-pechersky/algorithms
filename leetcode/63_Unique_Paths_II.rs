struct Solution {}

impl Solution {
    pub fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {
        let (n, m) = (obstacle_grid.len(), obstacle_grid[0].len());
        let mut dp = vec![0; m];
        dp[0] = 1;
        for i in 0..n {
            for j in 0..m {
                if obstacle_grid[i][j] == 0 {
                    if j > 0 {
                        dp[j] += dp[j - 1];
                    }
                } else {
                    dp[j] = 0;
                }
            }
        }
        dp[m - 1]
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
