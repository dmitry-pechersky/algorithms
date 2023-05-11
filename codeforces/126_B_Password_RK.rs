use std::io;

struct RK {
    q: u64,
    hashes: Vec<u64>,
    powers: Vec<u64>
}

impl RK {
    fn new(string: &[u8]) -> Self {
        Self::with(101, 39916801, string)
    }

    fn with(x: u64, q: u64, string: &[u8]) -> Self {
        let n = string.len();
        let (mut hashes, mut powers) = (vec![0; n], vec![0; n]);

        hashes[0] = (string[0] as u64) % q;
        for i in 1..n {
            hashes[i] = (hashes[i - 1] * x + string[i] as u64) % q;
        }

        powers[0] = 1;
        for i in 1..n {
            powers[i] = (powers[i - 1] * x) % q;
        }

        Self {q, hashes, powers}
    }    

    fn hash(&self, l: usize, r: usize) -> u64 {
        if l == 0 {
            self.hashes[r]
        } else {
            (self.q + self.hashes[r] - ((self.hashes[l - 1] * self.powers[r - l + 1]) % self.q)) % self.q
        }
    }

    fn hash_substring(&self, length: usize, hash: u64, ) -> bool {
        for l in 1..self.hashes.len() - length {
            if self.hash(l, l + length - 1) == hash {
                return true;
            }
        }
        false
    }
}

fn password_length(string: &[u8]) -> Option<usize> {
    let n = string.len();
    if n < 2 {
        return None;
    }
    let rk = RK::new(string);
    let mut substrings = vec![];
    for length in 1..n - 1 {
        let hash = rk.hash(0, length - 1);
        if hash == rk.hash(n - length, n - 1) {
            substrings.push((hash, length));
        }
    }

    if substrings.len() == 0 {
        return None
    }
    
    if let Some(idx) = binary_search(&substrings, &rk) {
        Some(substrings[idx].1)
    } else {
        None
    }
}

fn binary_search(strings: &Vec<(u64, usize)>, rk: &RK) -> Option<usize> { 
    let (mut left, mut right) = (0, strings.len() - 1);
    while left < right {
        let middle = (left + right + 1) / 2;
        let (hash, length) = strings[middle];
        if rk.hash_substring(length, hash) {
            left = middle;
        } else {
            right = middle - 1;
        }
    }
    let (hash, length) = strings[left];
    if rk.hash_substring(length, hash) {
        Some(left)
    } else {
        None
    }
}

fn main() { 
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    let string = buffer.trim();

    if let Some(length) = password_length(string.as_bytes()) {
        println!("{}", &string[0..length]);
    } else {
        println!("Just a legend");
    }
}


#[test]
fn test_rk() {
    let rk = RK::new("ababcabab".as_bytes());
    assert_eq!(rk.hash(0, 0), rk.hash(2, 2));
    assert_eq!(rk.hash(0, 1), rk.hash(5,6));
    assert_eq!(rk.hash(0, 3), rk.hash(5,8));    
    assert_ne!(rk.hash(0, 1), rk.hash(6, 7));
}


#[test]
fn test_password_length() {
    assert_eq!(Some(3), password_length("fixprefixsuffix".as_bytes()));
    assert_eq!(None, password_length("abcdabc".as_bytes()));
    assert_eq!(None, password_length("abcabcw".as_bytes()));
    assert_eq!(Some(2), password_length("qwqwqw".as_bytes()));
    assert_eq!(Some(2), password_length("abcabcab".as_bytes()));
    assert_eq!(None, password_length("a".as_bytes()));
    assert_eq!(Some(8), password_length("aaaaaaaaaa".as_bytes()));
}
