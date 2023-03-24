struct Solution;

impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let (mut i, mut j) = (0, matrix[0].len() as i32 - 1);
        while  i < matrix.len() && j >= 0 {
            if target == matrix[i][j as usize] {
                return true;
            } else if target < matrix[i][j as usize] {
                j -= 1;
            } else {
                i += 1
            }
        }
        false
    }
}

#[test]
fn test_1() {
    assert!(Solution::search_matrix(vec![vec![1,4,7,11,15],vec![2,5,8,12,19],vec![3,6,9,16,22],vec![10,13,14,17,24],vec![18,21,23,26,30]],5));
}

#[test]
fn test_2() {
    assert!(!Solution::search_matrix(vec![vec![1,4,7,11,15],vec![2,5,8,12,19],vec![3,6,9,16,22],vec![10,13,14,17,24],vec![18,21,23,26,30]], 20));
}
