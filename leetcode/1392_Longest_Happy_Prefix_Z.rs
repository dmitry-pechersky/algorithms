fn z_function(s: &[u8]) -> usize {
    let n = s.len();
    let (mut l, mut r) = (0, 0);
    let mut z= vec![0; n];
    for i in 1..n {
        if i < r {
            z[i] = z[i - l].min(r - i + 1);
        }
        while i + z[i] < n && s[i + z[i]] == s[z[i]] {
            z[i] += 1;
        }
        if i + z[i] - 1 > r {
            l = i;
            r = i + z[i] - 1
        }
        if i + z[i] == n {
            return z[i];
        }
    }
    0
}

struct Solution;

impl Solution {
    pub fn longest_prefix(s: String) -> String {
        let mut s = s;
        s.truncate(z_function(s.as_bytes()));
        s
    }
}

#[test]
fn test_longest_prefix() {
    
    assert_eq!(Solution::longest_prefix("level".to_string()), "l".to_string());
    assert_eq!(Solution::longest_prefix("ababab".to_string()), "abab".to_string());
    assert_eq!(Solution::longest_prefix("aaa".to_string()), "aa".to_string());    
    assert_eq!(Solution::longest_prefix("aaaab".to_string()), "".to_string());
}