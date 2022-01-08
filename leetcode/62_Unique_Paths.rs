struct Solution {}

impl Solution {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        let (m, n) = (m as usize, n as usize);
        let mut grid = [[1u32; 100]; 100];
        for i in 1..m {
            for j in 1..n {
                grid[i][j] = grid[i - 1][j] + grid[i][j - 1];
            }
        }
        grid[m - 1][n - 1] as i32
    }
}

#[cfg(test)]
mod unique_paths_test {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::unique_paths(3, 7), 28);
    }    

    #[test]
    fn test_2() {
        assert_eq!(Solution::unique_paths(3, 2), 3);
    }    

    #[test]
    fn test_3() {
        assert_eq!(Solution::unique_paths(7, 3), 28);
    }    

    #[test]
    fn test_4() {
        assert_eq!(Solution::unique_paths(3, 3), 6);
    }    
}
