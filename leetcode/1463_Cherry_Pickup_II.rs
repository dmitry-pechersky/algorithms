
struct Solution {}

impl Solution {
    pub fn cherry_pickup(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid[0].len();
        let mut dp = vec![vec![-1; m]; m];
        dp[0][m - 1] = grid[0][0] + grid[0][m - 1];
        for row in &grid[1..] {
            let mut dp_cur = vec![vec![-1; m]; m];
            for i in 0..m {
                for j in 0..m {
                    if dp[i][j] != -1 {
                        for &ni in [i as i32 - 1, i as i32, i as i32 + 1].iter() {
                            for &nj in [j as i32 - 1, j as i32, j as i32 + 1].iter() {
                                if ni >=0 && ni < m as i32 && nj >= 0 && nj < m as i32 {
                                    let v = dp[i][j] + row[ni as usize] + (if ni != nj { row[nj as usize] } else { 0 }); 
                                    if v > dp_cur[ni as usize][nj as usize] {
                                        dp_cur[ni as usize][nj as usize] = v;
                                    }

                                }
                            }
                        }
                    }
                }
            }
            dp = dp_cur;
        }
        *dp.iter().map(|a| a.iter().max().unwrap()).max().unwrap()
    }
}

mod cherry_pickup_test {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::cherry_pickup(vec![vec![3,1,1],vec![2,5,1],vec![1,5,5],vec![2,1,1]]), 24);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::cherry_pickup(vec![vec![1,0,0,0,0,0,1],vec![2,0,0,0,0,3,0],vec![2,0,9,0,0,0,0],vec![0,3,0,5,4,0,0],vec![1,0,2,3,0,0,6]]), 28);
    }

}