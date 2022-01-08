use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut dic: HashMap<String, Vec<String>> = HashMap::new();
        for s in strs {
            let mut bytes: Vec<u8> = s.bytes().collect();
            bytes.sort();
            let key: String = String::from_utf8(bytes).unwrap();
            dic.entry(key).or_insert(Vec::new()).push(s);

        }
        dic.values().cloned().collect()
    }
}

#[cfg(test)]
mod group_anagrams_test {
    use super::*;

    #[test]
    fn test_2() {
        assert_eq!(Solution::group_anagrams(vec!["".to_string()]) , vec![vec!["".to_string()]]);
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::group_anagrams(vec!["a".to_string()]) , vec![vec!["a".to_string()]]);
    }
}