struct Solution {}

impl Solution {
    pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
        const DIJ: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];
        fn rec(board: &Vec<Vec<char>>, word: &Vec<char>, used: &mut Vec<Vec<bool>>, i: usize, j: usize, idx: usize) -> bool {
            let (n, m) = (board.len() as i32, board[0].len() as i32);
            if word[idx] == board[i][j] {
                if idx == word.len() - 1 {
                    return true;
                }
                used[i][j] = true;
                for (di, dj) in &DIJ {
                    let (next_i , next_j) = (i as i32 + di, j as i32 + dj);
                    if 0 <= next_i && next_i < n && 0 <= next_j && next_j < m && !used[next_i as usize][next_j as usize] {
                        if rec(board, word, used, next_i as usize, next_j as usize, idx + 1) {
                            return true;
                        }
                    }
                }
                used[i][j] = false;
            }
            false
        }
        let (n, m) = (board.len(), board[0].len());
        let word = word.chars().collect();
        let mut used = vec![vec![false; m]; n];
        for i in 0..n {
            for j in 0..m {
                if rec(&board, &word, &mut used, i, j, 0) {
                    return true;
                }
            }
        }
        false
    }
}

#[cfg(test)]
mod exist_test {
    use super::*;

    #[test]
    fn test_1() {
        assert!(Solution::exist(vec![vec!['A','B','C','E'], vec!['S','F','C','S'], vec!['A','D','E','E']], "ABCCED".to_string()));
    }    

    #[test]
    fn test_2() {
        assert!(Solution::exist(vec![vec!['A','B','C','E'], vec!['S','F','C','S'], vec!['A','D','E','E']], "SEE".to_string()));
    }    

    #[test]
    fn test_3() {
        assert!(!Solution::exist(vec![vec!['A','B','C','E'], vec!['S','F','C','S'], vec!['A','D','E','E']], "ABCB".to_string()));
    }    

    #[test]
    fn test_4() {
        assert!(Solution::exist(vec![vec!['C','A','A'], vec!['A','A','A'], vec!['B','C','D']], "AAB".to_string()));
    }    
}