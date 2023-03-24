struct Solution {}

impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let (n, m) = (matrix.len(), matrix[0].len());
        let (mut left, mut right) = (0, n * m - 1);
        if target >= matrix[0][0] && target <= matrix[n - 1][m - 1] {
            while left <= right {
                let middle = (left + right) / 2;
                let middle_value = matrix[middle / m][middle % m];
                if middle_value == target {
                    return true;
                } else if middle_value < target {
                    left = middle + 1;
                } else {
                    right = middle - 1;
                }
            }
        }
        false
    }
}
