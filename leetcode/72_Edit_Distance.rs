struct Solution {}

impl Solution {
    pub fn min_distance(word1: String, word2: String) -> i32 {
        let (word1, word2) = (word1.as_bytes(), word2.as_bytes());
        let (n1, n2) = (word1.len(), word2.len());    
        let mut dp = vec![vec![0i32; n2 + 1]; n1 + 1];
        for i in 1..n1 + 1 {
            dp[i][0] = i as i32;
        }
        for i in 1..n2 + 1 { 
            dp[0][i] = i as i32;
        }
        for i in 1..n1 + 1 {
            for j in 1..n2 + 1 {
                dp[i][j] = dp[i - 1][j - 1] + if word1[i - 1] == word2[j - 1] { 0 } else {1};
                if dp[i][j - 1] + 1 < dp[i][j] {
                    dp[i][j] = dp[i][j - 1] + 1;
                }
                if dp[i - 1][j] + 1 < dp[i][j] {
                    dp[i][j] = dp[i - 1][j] + 1;
                }
            }
        }
        dp[n1][n2]
    }
}

#[cfg(test)]
mod min_distance_test {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::min_distance("horse".to_string(), "ros".to_string()), 3);
    }    

    #[test]
    fn test_2() {
        assert_eq!(Solution::min_distance("intention".to_string(), "execution".to_string()), 5);
    }    

}