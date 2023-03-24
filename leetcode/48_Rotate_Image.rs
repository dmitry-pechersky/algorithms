struct Solution;

use std::mem::swap;

impl Solution {
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        let n = matrix.len();
        for k in 0..n/2 {
            for l in k..n - k - 1 {
                let (mut i, mut j) = (l, k);
                let mut tmp = matrix[i][j];
                for _ in 0..4 {
                    let (next_i, next_j) = (j, n - 1 - i);
                    swap(&mut tmp, &mut matrix[next_i][next_j]);
                    i = next_i;
                    j = next_j;

                }
            }
        }
    }
}

#[test]
fn test_1() {
    let mut matrix = vec![vec![1,2,3],vec![4,5,6],vec![7,8,9]];
    Solution::rotate(&mut matrix);
    assert_eq!(matrix, vec![vec![7,4,1], vec![8,5,2], vec![9,6,3]]);
}

#[test]
fn test_2() {
    let mut matrix = vec![vec![5,1,9,11],vec![2,4,8,10],vec![13,3,6,7],vec![15,14,12,16]];
    Solution::rotate(&mut matrix);
    assert_eq!(matrix, vec![vec![15,13,2,5],vec![14,3,4,1],vec![12,6,8,9],vec![16,7,10,11]]);
}
