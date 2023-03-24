struct Solution {}

impl Solution {
    pub fn count_substrings(s: String) -> i32 {
        let s = s.as_bytes();
        let n = s.len();
        let mut cnt = 0;
        for i in 0..n {
            let (mut l, mut r) = (i, i);
            while r < n {
                if s[l] == s[r] {
                    cnt += 1;
                    if l == 0 {
                        break;
                    }
                    l -= 1;
                    r += 1;
                } else {
                    break;
                }
            }
            let (mut l, mut r) = (i, i + 1);
            while r < n {
                if s[l] == s[r] {
                    cnt += 1;
                    if l == 0 {
                        break;
                    }
                    l -= 1;
                    r += 1;
                } else {
                    break;
                }
            }            
        }
        cnt
    }
}

#[test]
fn test_1() {
    assert_eq!(3, Solution::count_substrings("abc".to_string()));
}

#[test]
fn test_2() {
    assert_eq!(6, Solution::count_substrings("aaa".to_string()));
}