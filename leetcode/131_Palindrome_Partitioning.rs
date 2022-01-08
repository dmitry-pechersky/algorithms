struct Solution {}

impl Solution {
    pub fn partition(s: String) -> Vec<Vec<String>> {
        fn _rec(s: &Vec<char>, start: usize, n: usize, partition: &mut Vec<(usize, usize)>, partitions: &mut Vec<Vec<String>>, dp: &Vec<Vec<bool>>) {
            if start >= n {
                partitions.push(partition.iter().map(| (i, j) | s[*i .. *j + 1].into_iter().collect()).collect());
            } else {
                for end in start..n {
                    if dp[start][end] {
                        partition.push((start, end));
                        _rec(s, end + 1, n, partition, partitions, dp);
                        partition.pop();
                    }
                }
            }
        }
        let s: Vec<char> = s.chars().collect();
        let n = s.len();
        let mut partitions = vec![];
        let mut partition = vec![];
        let mut dp = vec![vec![false; n]; n];
        for i in 0..n {
            dp[i][i] = true;
        }
        for l in 1..n {
            for i in 0..n-l {
                if s[i] == s[i + l] && (l == 1 || dp[i + 1][i + l - 1]) {
                    dp[i][i + l] = true;
                }
            }
        }
        _rec(&s, 0, n, &mut partition, &mut partitions, &dp);
        partitions
    }
}

mod partition_test {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            Solution::partition("aab".to_string()),
            vec![vec!["a".to_string(),"a".to_string(),"b".to_string()], vec!["aa".to_string(),"b".to_string()]]
        );
    }    

    #[test]
    fn test_2() {
        assert_eq!(
            Solution::partition("a".to_string()),
            vec![vec!["a".to_string()]]
        );
    }    
}    