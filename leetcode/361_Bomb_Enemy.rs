struct Solution {}

impl Solution {
    pub fn max_killed_enemies(grid: Vec<Vec<char>>) -> i32 {
        let (n, m) = (grid.len(), grid[0].len());
        let mut dp = vec![vec![0; m]; n];
        let mut max_cnt = 0;
        for i in 0..n {
            let mut cnt = 0;
            let mut start = 0;
            for j in 0..m {
                if grid[i][j] == 'E' {
                    cnt += 1;
                } else if grid[i][j] == 'W'{
                    for k in start..j {
                        if grid[i][k] == '0' {
                            dp[i][k] = cnt;
                        }
                    }
                    cnt = 0;
                    start = j + 1;
                }
            }
            for k in start..m {
                if grid[i][k] == '0' {
                    dp[i][k] = cnt;
                }
            }
        }
        for j in 0..m {
            let mut cnt = 0;
            let mut start = 0;
            for i in 0..n {
                if grid[i][j] == 'E' {
                    cnt += 1;
                } else if grid[i][j] == 'W'{
                    for k in start..i {
                        if grid[k][j] == '0' {
                            dp[k][j] += cnt;
                        }
                    }
                    cnt = 0;
                    start = i + 1;
                }
            }
            for k in start..n {
                if grid[k][j] == '0' {
                    dp[k][j] += cnt;
                }
            }
        }
        for i in 0..n {
            for j in 0..m {
                if grid[i][j] == '0' && dp[i][j] > max_cnt {
                    max_cnt = dp[i][j];
                }
            }
        }
        max_cnt        
    }
}

#[test]
fn test_1() {
    assert_eq!(
        3,
        Solution::max_killed_enemies(
            [
                ['0','E','0','0'],
                ['E','0','W','E'],
                ['0','E','0','0']
            ].iter().map(|array| array.to_vec()).collect::<Vec<_>>()
        )
    );    
}

#[test]
fn test_2() {
    assert_eq!(
        1,
        Solution::max_killed_enemies(
            [
                ['W','W','W'],
                ['0','0','0'],
                ['E','E','E']
            ].iter().map(|array| array.to_vec()).collect::<Vec<_>>()
        )
    );    
}