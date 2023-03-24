struct Solution {}

impl Solution {
    pub fn longest_palindrome_subseq(s: String) -> i32 {
        let s = s.as_bytes();
        let n = s.len();
        let mut dp_1 = (0..n as i32 + 1).collect::<Vec<_>>();
        let mut dp_2 = vec![0; n + 1];
        for i in 0..n {
            dp_2[0] = dp_1[0] + 1;
            for j in 0..n {
                dp_2[j + 1] = dp_1[j + 1].min(dp_2[j]) + 1;
                if s[j] == s[n - i - 1] && dp_1[j] < dp_2[j + 1] {
                    dp_2[j + 1] = dp_1[j];
                }
            }
            let tmp = dp_1;
            dp_1 = dp_2;
            dp_2 = tmp;
        }
        n as i32 - dp_1[n] / 2
    }
}

#[test]
fn test_0() {
    assert_eq!(1, Solution::longest_palindrome_subseq("abc".to_string()));
}

#[test]
fn test_1() {
    assert_eq!(4, Solution::longest_palindrome_subseq("bbbab".to_string()));
}

#[test]
fn test_2() {
    assert_eq!(2, Solution::longest_palindrome_subseq("cbbd".to_string()));
}