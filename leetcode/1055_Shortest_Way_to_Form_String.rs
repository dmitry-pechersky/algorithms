struct Solution {}

impl Solution {
    pub fn shortest_way(source: String, target: String) -> i32 {
        let (source, target) = (source.as_bytes(), target.as_bytes());
        let (source_n, target_n) = (source.len(), target.len());
        let mut idxs = [None; 26];
        let mut dp = vec![];
        for i in (0..source_n).rev() {
            let ch = source[i] - b'a';
            idxs[ch as usize] = Some(i as u16);
            dp.push(idxs.clone());
        }
        dp.reverse();
        let mut min_source_idx = 0;
        let mut min_number_subs = 1;
        let mut i = 0; 
        while i < target_n {
            let ch = target[i] - b'a';
            if min_source_idx >= source_n {
                min_source_idx = 0;
                min_number_subs += 1;
            }
            if let Some(idx) = dp[min_source_idx][ch as usize] {                
                min_source_idx = idx as usize + 1;
                i += 1;
            } else {
                if min_source_idx > 0 {
                    min_source_idx = 0;
                    min_number_subs += 1;
                } else {
                    return -1;
                }
            }
        }
        min_number_subs
    }
}

#[test]
fn test_0() {
    assert_eq!(3, Solution::shortest_way("aa".to_string(), "aaaaa".to_string()));
}

#[test]
fn test_1() {
    assert_eq!(2, Solution::shortest_way("abc".to_string(), "abcbc".to_string()));
}

#[test]
fn test_2() {
    assert_eq!(-1, Solution::shortest_way("abc".to_string(), "acdbc".to_string()));
}

#[test]
fn test_3() {
    assert_eq!(3, Solution::shortest_way("xyz".to_string(), "xzyxz".to_string()));
}