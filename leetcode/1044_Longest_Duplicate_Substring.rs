use  std::collections::HashSet;
struct Solution;

struct RK{
    q: u128,
    hashes: Vec<u128>,
    powers: Vec<u128>
}

impl RK {
    fn new(string: &[u8]) -> Self {
        let n = string.len();
        let (q, x) = (2u128.pow(63) - 1, 10192726);
        let mut hashes = vec![0; n];
        let mut powers = vec![0; n];

        hashes[0] = string[0] as u128;
        for i in 1..n {
            hashes[i] = (hashes[i - 1] * x + string[i] as u128) % q;
        }

        powers[0] = 1;
        for i in 1..n {
            powers[i] = (powers[i - 1] * x) % q;
        }
        
        Self {q, hashes, powers}
    }

    fn hash(&self, left: usize, right: usize) -> u128 {
        if left == 0 {
            self.hashes[right]
        } else {
            (self.hashes[right] + self.q - (self.hashes[left - 1] * self.powers[right - left  + 1]) % self.q) % self.q
        }
    }
}

fn binary_search<F: Fn(usize) -> bool>(left: usize, right: usize, predicate: F) -> Option<usize> {
    let (mut left, mut right) = (left, right);
    if ! predicate(left) {
        return None
    }
    while left < right {
        let middle = left + (right - left + 1) / 2;
        if predicate(middle) {
            left = middle;
        } else {
            right = middle - 1;
        }
    }
    Some(left)
}

fn find_duplicate(rk: &RK, length: usize, n: usize) -> Option<usize> {
    let mut set = HashSet::new();
    for i in 0..=n - length {
        let hash = rk.hash(i, i + length - 1);
        if set.contains(&hash) {
            return Some(i);
        }
        set.insert(hash);
    }
    None
}

impl Solution {
    pub fn longest_dup_substring(s: String) -> String {
        let n = s.len();
        let rk = RK::new(s.as_bytes());
        if let Some(max_length) = binary_search(1, s.len() - 1, |length| { find_duplicate(&rk, length, n).is_some() }) {
            let idx = find_duplicate(&rk, max_length, n).unwrap();
            s[idx..idx + max_length].to_string()
        } else {
            "".to_string()
        }
        
    }
}

#[test] 
fn tetst_rk() {
    let rk = RK::new("ababcab".as_bytes());
    assert_eq!(rk.hash(0, 0), rk.hash(2, 2));  
    assert_eq!(rk.hash(0, 0), rk.hash(5, 5));
    assert_ne!(rk.hash(0, 0), rk.hash(3, 3));
    assert_eq!(rk.hash(0, 1), rk.hash(5, 6));
}

#[test]
fn test_longest_dup_substring() {
    assert_eq!("ana".to_string(), Solution::longest_dup_substring("banana".to_string()));
    assert_eq!("".to_string(), Solution::longest_dup_substring("abcd".to_string()));
}
