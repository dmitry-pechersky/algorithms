struct Solution {}

impl Solution {
    pub fn longest_increasing_path(matrix: Vec<Vec<i32>>) -> i32 {
        fn rec(i: usize, j: usize, n: usize, m: usize, matrix: &Vec<Vec<i32>>, dp: &mut Vec<Vec<i32>>) -> i32 {
            if dp[i][j] == 0 {
                let mut max_next_length = 0;
                for (i1, j1) in [(i as i32 + 1, j as i32), (i as i32, j as i32 + 1), (i as i32 - 1, j as i32), (i as i32, j as i32 - 1)] {
                    if 0 <= i1 && i1 < n as i32 && 0 <= j1 && j1 < m as i32 && matrix[i][j] < matrix[i1 as usize][j1 as usize] {
                        max_next_length = max_next_length.max(rec(i1 as usize, j1 as usize, n, m, matrix, dp));
                    }
                }
                dp[i][j] = max_next_length + 1;
            }
            dp[i][j]
        }

        let (n, m) = (matrix.len(), matrix[0].len());
        let mut max_length = 0;
        let mut dp = vec![vec![0; m]; n];
        for i in 0..n {
            for j in 0..m {
                max_length = max_length.max(rec(i, j, n, m, &matrix, &mut dp));
            }
        }
        max_length
    }
}

#[test]
fn test_1() {
    assert_eq!(4, Solution::longest_increasing_path(vec![vec![9,9,4],vec![6,6,8],vec![2,1,1]]));
}

#[test]
fn test_2() {
    assert_eq!(4, Solution::longest_increasing_path(vec![vec![3,4,5],vec![3,2,6],vec![2,2,1]]));
}

#[test]
fn test_3() {
    assert_eq!(1, Solution::longest_increasing_path(vec![vec![1]]));
}