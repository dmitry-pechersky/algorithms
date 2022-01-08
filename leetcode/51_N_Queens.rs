struct Solution {}

impl Solution {
    pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
        const MAX_N: usize = 9;
        let mut column = [false; MAX_N];
        let mut left_diag = [false; 2 * MAX_N];
        let mut right_diag = [false; 2 * MAX_N];
        let mut board = vec![vec!['.'; n as usize]; n as usize];
        let mut boards = Vec::<Vec<String>>::new();
        fn _rec(n: usize, i: usize, column: &mut [bool], left_diag: &mut [bool], right_diag: &mut [bool], board: &mut Vec<Vec<char>>, boards: &mut Vec<Vec<String>>) {
            if i >= n {
                boards.push(board.iter().map(|v| v.iter().collect::<String>()).collect());
            } else {
                for j in 0..n {
                    if !(column[j] || left_diag[n + j - i] || right_diag[j + i]) {
                        column[j] = true;
                        left_diag[n + j - i] = true;
                        right_diag[j + i] = true;
                        board[i][j] = 'Q';
                        _rec(n, i + 1, column, left_diag, right_diag, board, boards);
                        board[i][j] = '.';
                        column[j] = false;
                        left_diag[n + j - i] = false;
                        right_diag[j + i] = false;
                    }
                }
            }
        }
        _rec(n as usize, 0, &mut column, &mut left_diag, &mut right_diag, &mut board, &mut boards);
        boards
    }
}

#[cfg(test)]
mod solve_n_queens_test {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            Solution::solve_n_queens(4), 
            vec![
                vec![".Q..".to_string(),"...Q".to_string(),"Q...".to_string(),"..Q.".to_string()],
                vec!["..Q.".to_string(),"Q...".to_string(),"...Q".to_string(),".Q..".to_string()]]
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::solve_n_queens(1), vec![vec!["Q".to_string()]]);
    }    
}