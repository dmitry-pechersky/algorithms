struct Solution { }


impl Solution {
    fn _match(s: &[u8], p: &[u8], i: usize, j: usize, cache: &mut Vec<Vec<Option<bool>>>) -> bool {
        let (s_n, p_n) = (s.len(), p.len());
        if i < s_n && j < p_n && cache[i][j].is_some() {
            return cache[i][j].unwrap();
        } 
        if i == s_n && j == p_n {
            return true;
        }
        let mut res = false;
        let asterisk = j + 1 < p_n && p[j + 1] == '*' as u8;
        if asterisk {
            res = res || Solution::_match(s, p, i, j + 2, cache);
        } 
        if i < s_n && j < p_n {
            if s[i] == p[j] || p[j] == ('.' as u8) {
                if asterisk {
                    res = res || Solution::_match(s, p, i + 1, j, cache);
                } else {
                    res = res || Solution::_match(s, p, i + 1, j + 1, cache);
                }
            }
        }
        if i < s_n && j < p_n { 
            cache[i][j] = Some(res);
        }
        res
    }

    pub fn is_match(s: String, p: String) -> bool {
        let mut cache: Vec<Vec<Option<bool>>> = vec![vec![None; p.len()]; s.len()];
        Solution::_match(s.as_bytes(), p.as_bytes(), 0, 0, &mut cache)
    }
}

#[cfg(test)]
mod match_test {
    use super::*;

    #[test]
    fn test_1() {
        assert!(!Solution::is_match(String::from("aa"), String::from("a")));
    }

    #[test]
    fn test_2() {
        assert!(Solution::is_match(String::from("aa"), String::from("a*")));
    }

    #[test]
    fn test_3() {
        assert!(Solution::is_match(String::from("ab"), String::from(".*")));
    }

    #[test]
    fn test_4() {
        assert!(Solution::is_match(String::from("aab"), String::from("c*a*b")));
    }

    #[test]
    fn test_5() {
        assert!( ! Solution::is_match(String::from("mississippi"), String::from("mis*is*p*.")));
    }
}