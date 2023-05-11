fn prefix_function(s: &[u8]) -> Vec<u32> {
    let n = s.len();
    let mut kmp = vec![0; n]; 
    for i in 1..n {
        let mut j = i;
        while j > 0 && s[kmp[j - 1] as usize] != s[i] {
            j = kmp[j - 1] as usize;
        }
        if j > 0 {
            kmp[i] = kmp[j - 1] + 1;
        }
    }
    kmp
}

struct Solution;

impl Solution {
    pub fn longest_prefix(s: String) -> String {
        let kmp = prefix_function(s.as_bytes());
        s[0..*kmp.last().unwrap() as usize].to_string()
    }
}

#[test]
fn test_prefix_function() {
    assert_eq!(prefix_function("ababab".as_bytes()), vec![0,0,1,2,3,4]);
    assert_eq!(prefix_function("abcac".as_bytes()), vec![0,0,0,1,0]);
}

#[test]
fn test_longest_prefix() {
    assert_eq!(Solution::longest_prefix("level".to_string()), "l".to_string());
    assert_eq!(Solution::longest_prefix("ababab".to_string()), "abab".to_string());
    assert_eq!(Solution::longest_prefix("aaa".to_string()), "aa".to_string());
}