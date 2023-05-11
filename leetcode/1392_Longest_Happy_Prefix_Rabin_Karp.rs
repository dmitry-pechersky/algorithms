fn rk_hash_function(s: &[u8]) -> impl Fn(usize, usize) -> u32 {
    const X: u64 = 103;
    const  Q: u64 = (1 << 31) - 1;
    let n = s.len();
    let mut powers = vec![0; n];
    let mut hashes = vec![0; n];
    powers[0] = 1;
    for i in 1..n {
        powers[i] = ((powers[i - 1] as u64 * X) % Q) as u32;
    }
    hashes[0] = s[0] as u32;
    for i in 1..n {
        hashes[i] = (((hashes[i - 1] as u64 * X) % Q + s[i] as u64) % Q) as u32;
    }

    move |l: usize, r: usize| -> u32 {
        if l == 0 {
            hashes[r]
        } else {
            ((hashes[r] as u64 + Q -  (powers[r - l + 1] as u64 *  hashes[l - 1] as u64) % Q) % Q) as u32
        }
    }
}

struct Solution;

impl Solution {
    pub fn longest_prefix(s: String) -> String {
        let n = s.len();
        let hash_function = rk_hash_function(s.as_bytes());
        for length in (1..n).rev()  {
            if hash_function(0, length - 1) == hash_function(n - length, n - 1) {
                return s[0..length].to_string()
            }
        }
        String::new()
    }
}

#[test]
fn test_hash() {
    let s = "abcab".as_bytes();
    let hash_function = rk_hash_function(s);
    assert_eq!(hash_function(0, 1), hash_function(3, 4));
    assert_ne!(hash_function(0, 1), hash_function(2, 3));
}

#[test]
fn test_longest_prefix() {
    assert_eq!(Solution::longest_prefix("level".to_string()), "l".to_string());
    assert_eq!(Solution::longest_prefix("ababab".to_string()), "abab".to_string());
    assert_eq!(Solution::longest_prefix("aaa".to_string()), "aa".to_string());
}