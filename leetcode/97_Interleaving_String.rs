struct Solution {}

impl Solution {
    pub fn is_interleave(s1: String, s2: String, s3: String) -> bool {
        let (s1, s2, s3) = (s1.chars().collect::<Vec<char>>(), s2.chars().collect::<Vec<char>>(), s3.chars().collect::<Vec<char>>());
        if s1.len() + s2.len() != s3.len() {
            return false;
        }
        let mut dp = vec![false; s2.len() + 1];
        dp[0] = true;
        for j in 1..=s2.len() {
            dp[j] = s2[j - 1] == s3[j - 1];
            if ! dp[j] { 
                break 
            }
        }
        for i in 1..=s1.len() {
            for j in 0..=s2.len() {
                dp[j] = (j > 0 && dp[j - 1] && s2[j - 1] == s3[i + j - 1]) || (dp[j] && s1[i - 1] == s3[i + j - 1]);
            }
        }
        dp[s2.len()]
    }
}

#[cfg(test)]
mod is_interleave_test {
    use super::*;

    #[test]
    fn test_1() {
        assert!(Solution::is_interleave("aabcc".to_string(), "dbbca".to_string(), "aadbbcbcac".to_string()));
    }    
    #[test]
    fn test_2() {
        assert!(!Solution::is_interleave("aabcc".to_string(), "dbbca".to_string(), "aadbbbaccc".to_string()));
    }    

    #[test]
    fn test_3() {
        assert!(Solution::is_interleave("".to_string(), "".to_string(), "".to_string()));
    }  
}