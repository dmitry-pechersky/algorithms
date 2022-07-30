use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn num_distinct(s: String, t: String) -> i32 {
        fn rec(s: &[u8], t: &[u8], i: usize, j: usize, dp: &mut HashMap<(usize, usize), i32>) -> i32 {
            let (s_n, t_n) = (s.len(), t.len());
            if i <= s_n && j == t_n {
                return 1;
            } else if i >= s_n || j >= t_n || (s_n - i) < (t_n - j) {
                return 0;
            }
            if ! dp.contains_key(&(i, j)) {
                let mut val = rec(s, t, i + 1, j, dp);
                if s[i] == t[j] {
                    val += rec(s, t, i + 1, j + 1, dp);
                }
                dp.insert((i, j), val);
            }
            dp[&(i, j)]
        }
        let (s, t) = (s.as_bytes(), t.as_bytes());
        let mut dp = HashMap::new();
        rec(s, t, 0, 0, &mut dp)
    }
}


mod path_sum_test {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::num_distinct("rabbbit".to_string(), "rabbit".to_string()), 3);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::num_distinct("babgbag".to_string(), "bag".to_string()), 5);
    }
}