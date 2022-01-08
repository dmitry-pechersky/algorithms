struct Solution {}

impl Solution {
    pub fn total_n_queens(n: i32) -> i32 {
        const MAX_N: usize = 20;
        let mut column_used = [false; MAX_N];
        let mut left_diag_used = [false; MAX_N * 2];
        let mut right_diag_used = [false; MAX_N * 2];
        let mut cnt = 0;

        fn _rec(n: usize, i: usize, column_used: &mut [bool], left_diag_used:  &mut  [bool], right_diag_used:  &mut [bool], cnt: &mut usize) {
            if i >= n {
                *cnt += 1;
            } else {
                for j in 0..n {
                    if !(column_used[j] || left_diag_used[n + j - i] || right_diag_used[j + i]) {
                        column_used[j] = true;
                        left_diag_used[n + j - i] = true;
                        right_diag_used[j + i] = true;
                        _rec(n, i + 1, column_used, left_diag_used, right_diag_used, cnt);
                        column_used[j] = false;
                        left_diag_used[n + j - i] = false;
                        right_diag_used[j + i] = false;
                    }
                }
            }
        }
        _rec(n as usize, 0, &mut column_used, &mut left_diag_used, &mut right_diag_used, &mut cnt);
        cnt as i32
    }
}

#[cfg(test)]
mod total_n_queens_test {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::total_n_queens(14), 2);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::total_n_queens(1), 1);
    }
}