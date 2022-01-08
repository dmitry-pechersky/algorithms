struct Solution {}

impl Solution {
    pub fn simplify_path(path: String) -> String {
        let mut stack: Vec<&str> = vec![];
        for token in path.split('/') {
            if token != "" && token != "." {
                if token != ".." {
                    stack.push(token);
                }else if ! stack.is_empty() {
                    stack.pop();
                }
            }
        }
        "/".to_string() + &stack.join("/")
    }
}

#[cfg(test)]
mod simplify_path_test {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::simplify_path("/home/".to_string()), "/home".to_string());
    }    

    #[test]
    fn test_2() {
        assert_eq!(Solution::simplify_path("/../".to_string()), "/".to_string());
    }    

    #[test]
    fn test_3() {
        assert_eq!(Solution::simplify_path("/home//foo/".to_string()), "/home/foo".to_string());
    }    

    #[test]
    fn test_4() {
        assert_eq!(Solution::simplify_path("/a/./b/../../c/".to_string()), "/c".to_string());
    }    
}
