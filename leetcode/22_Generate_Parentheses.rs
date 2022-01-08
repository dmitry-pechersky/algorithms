use std::collections::HashSet;
use std::iter::FromIterator;

struct Solution {}

impl Solution {
    fn _rec(n: i32, left_n: i32, right_n: i32, combination: &mut String, combinations: &mut Vec<String>) {
        if left_n == n && right_n == n {
            combinations.push(combination.clone());
        } else {
            if left_n < n {
                combination.push('(');
                Self::_rec(n, left_n + 1, right_n, combination, combinations);
                combination.pop();
            }
            if right_n < left_n {
                combination.push(')');
                Self::_rec(n, left_n, right_n + 1, combination, combinations);
                combination.pop();
            }
        }
    }

    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut combinations = Vec::<String>::new();
        let mut combination = String::new();
        Self::_rec(n, 0, 0, &mut combination, &mut combinations);   
        combinations
    }
}

#[cfg(test)]
mod generate_parenthesis_test {
    use super::*;

    #[test]
    fn test_1() {    
        assert_eq!(
            HashSet::<String>::from_iter(Solution::generate_parenthesis(1)),
            HashSet::from_iter(vec!["()".to_string()])
        );
    }

    #[test]
    fn test_2() {    
        assert_eq!(
            HashSet::<String>::from_iter(Solution::generate_parenthesis(3)),
            HashSet::from_iter(vec![
                "((()))".to_string(),
                "(()())".to_string(),
                "(())()".to_string(),
                "()(())".to_string(),
                "()()()".to_string()])
        );
    }
}
