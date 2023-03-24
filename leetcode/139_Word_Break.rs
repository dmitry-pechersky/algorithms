use std::collections::{HashSet};

struct Solution {}

impl Solution {
    pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
        fn rec(s: &Vec<char>, idx: usize, dic: &HashSet<String>, dp: &mut Vec<bool>) -> bool {
            let n = s.len();
            if idx == n {
                true
            } else if dp[idx] {
                false
            } else {
                let mut word = String::new();
                for i in idx..n.min(idx + 20) {
                    word.push(s[i]);
                    if dic.contains(&word) {
                        if rec(s, i + 1, dic, dp) {
                            return true;
                        } 
                    }
                } 
                dp[idx] = true;   
                false
            }
        }

        let s = s.chars().collect::<Vec<_>>();
        rec(
            & s,
            0, 
            & word_dict.into_iter().collect::<HashSet<_>>(),
            &mut vec![false; s.len()]
        )
    }
}

#[test]
fn test_1() {
    assert_eq!(true, Solution::word_break("leetcode".to_string(), vec!["leet".to_string() ,"code".to_string()]));
}
