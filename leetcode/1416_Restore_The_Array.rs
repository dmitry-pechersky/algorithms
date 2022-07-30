struct Solution {}

impl Solution {
    pub fn number_of_arrays(s: String, k: i32) -> i32 {
        fn rec(s: &Vec<u8>, i: usize, n: usize, k: u64, dp: &mut Vec<Option<u32>>) -> u32  {
            if let Some(res) = dp[i] {
                res
            } else {
                let mut res = 0;
                if s[i] != 0 {
                    let mut value = 0;
                    for j in i..n {
                        value = value * 10 + s[j] as u64;
                        if value > k {
                            break;
                        }
                        res = (res + rec(s, j + 1, n, k, dp))  % 1000000007;
                    }
                }
                dp[i] = Some(res);
                res 
            }            
        }
        let mut s = s.into_bytes();
        for x in s.iter_mut() {
            *x -= 48;
        }
        let n = s.len();
        let mut dp = vec![None; n + 1];
        dp[n] = Some(1);
        rec(&s, 0, n, k as u64, &mut dp) as i32
    }
}

#[test]
fn test_1() {
    assert_eq!(1, Solution::number_of_arrays("1000".to_string(), 10000));
}

#[test]
fn test_2() {
    assert_eq!(0, Solution::number_of_arrays("1000".to_string(), 10));
}

#[test]
fn test_3() {
    assert_eq!(8, Solution::number_of_arrays("1317".to_string(), 2000));
}