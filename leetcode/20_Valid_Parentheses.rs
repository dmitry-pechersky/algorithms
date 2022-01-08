struct Solution {}

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack = Vec::<char>::new(); 
        for ch in s.chars() {
            if ch == '(' || ch == '{' || ch == '[' {
                stack.push(ch);
            } else if stack.len() == 0 {
                return false;
            } else {
                let left = stack.pop().unwrap();
                if !((ch == ')' && left == '(') || (ch == ']' && left == '[') || (ch == '}' && left == '{')) {
                    return false;
                }
            }
        }
        stack.is_empty()
    }
}

#[cfg(test)]
mod valid_parentheses_test {
    use super::*;

    #[test]
    fn test_1() {
        assert!(Solution::is_valid("()".to_string()));
    }

    #[test]
    fn test_2() {
        assert!(Solution::is_valid("()[]{}".to_string()));
    }

    #[test]
    fn test_3() {
        assert!(!Solution::is_valid("(]".to_string()));
    }

    #[test]
    fn test_4() {
        assert!(!Solution::is_valid("([)]".to_string()));
    }

    #[test]
    fn test_5() {
        assert!(Solution::is_valid("{[]}".to_string()));
    }

}