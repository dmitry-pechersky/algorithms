use std::collections::HashSet;

struct Solution {}

impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        let s: Vec<char> = s.chars().collect();
        let p: Vec<char> = p.chars().collect();
        let (s_n, p_n) = (s.len(), p.len());
        let mut dp: HashSet<(usize, usize)> = HashSet::new();
        let mut stack: Vec<(usize, usize)> = vec![(0, 0)];
        while stack.len() > 0 {
            let (i, j) = stack.pop().unwrap();
            if !dp.contains(&(i, j)) {
                dp.insert((i, j));
                if i < s_n && j < p_n {
                    if s[i] == p[j] || p[j] == '?' {
                        stack.push((i + 1, j + 1));
                    } else if p[j] == '*' {
                        stack.push((i + 1, j + 1));
                        stack.push((i + 1, j));
                        stack.push((i, j + 1));
                    }
                } else if i == s_n && j < p_n && p[j] == '*' {
                    stack.push((i, j + 1));

                } else if i == s_n && j == p_n {
                    return true;
                }
            }
        }
        false
    }
}

#[cfg(test)]
mod is_match_test {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::is_match("aa".to_string(), "a".to_string()), false);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::is_match("aa".to_string(), "*".to_string()), true);
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::is_match("cb".to_string(), "?a".to_string()), false);
    }

    #[test]
    fn test_4() {
        assert_eq!(Solution::is_match("adceb".to_string(), "*a*b".to_string()), true);
    }

    #[test]
    fn test_5() {
        assert_eq!(Solution::is_match("acdcb".to_string(), "a*c?b".to_string()), false);
    }

    #[test]
    fn test_6() {
        assert_eq!(Solution::is_match("".to_string(), "***".to_string()), true);
    }
}