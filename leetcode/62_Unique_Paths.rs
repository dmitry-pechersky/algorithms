struct Solution {}

impl Solution {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        let (n, m) = (n as usize, m as usize);
        let mut dp = vec![1; n];
        for _ in 1..m {
            for i in 1..n {
                dp[i] += dp[i - 1];
            }
        }       
        dp[n - 1] 
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
