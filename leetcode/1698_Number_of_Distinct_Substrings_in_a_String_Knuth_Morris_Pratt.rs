fn kmp(chars: &[u8], prefix_func: &mut [u16]) -> u16 {
    let n = chars.len();
    prefix_func[0] = 0;
    let mut max_len = 0;
    for i in 1..n {
        let mut j = i;
        while j > 0 && chars[prefix_func[j - 1] as usize] != chars[i] {
            j = prefix_func[j - 1] as usize;
        }
        if j > 0 {
            prefix_func[i] = prefix_func[j - 1] + 1;
            max_len = prefix_func[i].max(max_len);
        } else {
            prefix_func[i] = 0;
        }
    }
    max_len
}

struct Solution;

impl Solution {
    pub fn count_distinct(s: String) -> i32 {
        let s = s.as_bytes();
        let n = s.len();
        let mut cnt = 0;
        let mut prefix_func = vec![0; n];
        for i in 0..n {
            let max_len = kmp(&s[i..n], &mut prefix_func);
            cnt += (n - i) - max_len as usize;
        }
        cnt as i32
    }
}

#[test]
fn test_count_distinct() {
    assert_eq!(Solution::count_distinct("aabbaba".to_string()), 21);
    assert_eq!(Solution::count_distinct("abcdefg".to_string()), 28);
}
