struct Solution;

impl Solution {
    pub fn longest_prefix(s: String) -> String {
        const X: u64 = 103; 
        const Q: u64 = (1 << 31) - 1;
        let chars = s.as_bytes();
        let n = chars.len();
        let (mut left_idx, mut right_idx) = (0, n - 1);
        let (mut left_hash, mut right_hash) = (0, 0);
        let mut x_power = 1;
        let mut max_length = 0;
        while left_idx < n - 1  {
            left_hash = (((left_hash * X) % Q) + chars[left_idx] as u64) % Q;
            right_hash = ((chars[right_idx] as u64 * x_power) % Q + right_hash) % Q;
            x_power = (x_power * X) % Q;
            left_idx += 1;
            right_idx -= 1;

            if left_hash == right_hash {
                max_length = left_idx;
            }
        }
        let mut s = s;
        s.truncate(max_length);
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
