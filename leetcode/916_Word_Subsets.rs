struct Solution {}

#[derive(PartialEq)]
struct Chars {
    chars: [u8; Chars::CHARS_N],
}
impl Chars {
    const CHARS_N: usize = (b'z' - b'a' + 1) as usize;

    fn new() -> Self {
        Self { chars: [0; Chars::CHARS_N] }
    }
    
    fn intersection(&self, chars: &Chars) -> Self {
        let mut res = Self::new();
        for i in 0..Chars::CHARS_N {
            res.chars[i] = self.chars[i].min(chars.chars[i]);
        }
        res
    }

    fn union(&self, chars: &Chars) -> Self {
        let mut res = Self::new();
        for i in 0..Chars::CHARS_N {
            res.chars[i] = self.chars[i].max(chars.chars[i]);
        }
        res
    }
}

impl From<&String> for Chars {
    fn from(str: &String) -> Self {
        let mut res = Self::new();
        for ch in str.as_bytes().iter().map(|ch| ch - b'a') {
            res.chars[ch as usize] += 1;
        }
        res
    }
}

impl Solution {
    pub fn word_subsets(words1: Vec<String>, words2: Vec<String>) -> Vec<String> {
        let mut res = Vec::new();
        let mut chars2: Chars = Chars::new();
        for word in words2.iter() {
            chars2 = chars2.union(&word.into());
        }
        for word in words1 {
            let chars1: Chars = (&word).into();
            if chars2 == chars2.intersection(&chars1) {
                res.push(word);
            }
        }
        res
    }
}

#[test]
fn test_1() {
    assert_eq!(
        Solution::word_subsets(
            ["amazon","apple","facebook","google","leetcode"].iter().map(|str| str.to_string()).collect::<Vec<_>>(),
            ["e","o"].iter().map(|str| str.to_string()).collect::<Vec<_>>()
        ),
        ["facebook","google","leetcode"].iter().map(|str| str.to_string()).collect::<Vec<_>>()
    );
}

#[test]
fn test_2() {
    assert_eq!(
        Solution::word_subsets(
            ["amazon","apple","facebook","google","leetcode"].iter().map(|str| str.to_string()).collect::<Vec<_>>(),
            ["l","e"].iter().map(|str| str.to_string()).collect::<Vec<_>>()
        ),
        ["apple","google","leetcode"].iter().map(|str| str.to_string()).collect::<Vec<_>>()
    );
}
