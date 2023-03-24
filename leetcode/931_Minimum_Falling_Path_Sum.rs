struct Solution {}

impl Solution {
    pub fn min_falling_path_sum(matrix: Vec<Vec<i32>>) -> i32 {
        let n = matrix.len();
        let mut matrix = matrix;
        for i in 1..n {
            for j in 0..n {
                let mut min_value = matrix[i - 1][j];
                if j > 0 && matrix[i - 1][j - 1] < min_value {
                    min_value = matrix[i - 1][j - 1];
                }
                if j < n - 1 && matrix[i - 1][j + 1] < min_value {
                    min_value = matrix[i - 1][j + 1];
                }
                matrix[i][j] += min_value;
            }
        }
        *matrix[n - 1].iter().min().unwrap()
    }
}

#[test] 
fn test_1() {
    assert_eq!(13, Solution::min_falling_path_sum(vec![vec![2,1,3],vec![6,5,4],vec![7,8,9]]));
}

#[test] 
fn test_2() {
    assert_eq!(-59, Solution::min_falling_path_sum(vec![vec![-19,57],vec![-40,-5]]));
}
