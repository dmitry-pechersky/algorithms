use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn is_scramble(s1: String, s2: String) -> bool {
        let s1: Vec<char> = s1.chars().collect();
        let s2: Vec<char> = s2.chars().collect();
        let mut dp = HashMap::new();
        fn _rec(s1: &Vec<char>, s2: &Vec<char>, i1: usize, i2: usize, j: usize, dp: &mut HashMap<(usize, usize, usize), bool>) -> bool {
            if i1 == i2 {
                s1[i1] == s2[j]
            } else {
                if ! dp.contains_key(&(i1, i2, j)) {
                    let mut res = false;
                    for k in i1..i2 {
                        if _rec(s1, s2, i1, k, j, dp) && _rec(s1, s2, k + 1, i2, j + k - i1 + 1, dp) {
                            res = true;
                            break
                        }
                        if _rec(s1, s2, i1, k, j + i2 - k, dp) && _rec(s1, s2, k + 1, i2, j, dp) {
                            res = true;
                            break
                        }
                    }
                    dp.insert((i1, i2, j), res);
                }
                *dp.get(&(i1, i2, j)).unwrap()
            }
        }
        _rec(&s1, &s2, 0, s1.len() - 1, 0, &mut dp)
    }
}

#[cfg(test)]
mod is_scramble_test {
    use super::*;

    #[test]
    fn test_2() {
        assert!(!Solution::is_scramble("abcde".to_string(), "caebd".to_string()));
    }    

    #[test]
    fn test_1() {
        assert!(Solution::is_scramble("great".to_string(), "rgeat".to_string()));
    }    

    #[test]
    fn test_3() {
        assert!(Solution::is_scramble("a".to_string(), "a".to_string()));
    }    
}