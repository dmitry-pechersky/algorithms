struct Solution {}

impl Solution {
    pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
        let (text1, text2) = (text1.as_bytes(), text2.as_bytes());
        let (n1, n2) = (text1.len(), text2.len());
        let mut dp_1 = vec![0;n1];
        let mut dp_2 = vec![0;n1];
        for i in 0..n2 {            
            dp_2[0] = if text2[i] == text1[0] { 1 } else { dp_1[0] };
            for j in 1..n1 {
                dp_2[j] = dp_1[j].max(dp_2[j - 1]).max(dp_1[j - 1] + if text2[i] == text1[j] { 1 } else { 0 });
            }
            let tmp = dp_2;
            dp_2 = dp_1;
            dp_1 = tmp;
        }
        dp_1[n1 - 1]
    }
}

#[test]
fn test_1() {
    assert_eq!(3, Solution::longest_common_subsequence("abcde".to_string(), "ace".to_string()));
    assert_eq!(3, Solution::longest_common_subsequence("abc".to_string(), "abc".to_string()));
    assert_eq!(0, Solution::longest_common_subsequence("abc".to_string(), "def".to_string()));
}