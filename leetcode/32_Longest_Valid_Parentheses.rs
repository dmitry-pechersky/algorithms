struct Solution {}

impl Solution {
    pub fn longest_valid_parentheses(s: String) -> i32 {
        let s: Vec<char> = s.chars().collect();
        let n = s.len();
        let mut stack: Vec<usize> = vec![];
        let mut valids = vec![false; n];
        for i in 0..n {
            if s[i] == '(' {
                stack.push(i);
            } else if stack.len() > 0 {
                valids[stack.pop().unwrap()] = true;
                valids[i] = true;
            } 
        }
        let (mut cnt, mut max_cnt) = (0, 0);
        for valid in valids{
            if valid {
                cnt += 1;
                if cnt > max_cnt { max_cnt = cnt; }
            } else {
                cnt = 0;
            }
        }
        max_cnt
    }
}

#[cfg(test)]
mod longest_valid_parentheses_test {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::longest_valid_parentheses("(()".to_string()), 2);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::longest_valid_parentheses(")()())".to_string()), 4);
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::longest_valid_parentheses("".to_string()), 0);
    }
}
