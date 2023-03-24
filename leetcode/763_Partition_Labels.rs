struct Solution {}

impl Solution {
    pub fn partition_labels(s: String) -> Vec<i32> {
        let s = s.as_bytes();
        let mut idx = [0; 26];
        let mut res = vec![];
        for (i, ch) in s.iter().map(|v| v - b'a').enumerate() {
            idx[ch as usize] = i; 
        }
        let (mut start, mut end) = (0, 0);
        for (i, ch) in s.iter().map(|v| v - b'a').enumerate() {
            end = end.max(idx[ch as usize]);
            if i == end {
                res.push((end - start) as i32 + 1);
                start = i + 1;
            }
        }
        res
    }
}

#[test]
fn tast_1() {
    assert_eq!(Solution::partition_labels("ababcbacadefegdehijhklij".to_string()), vec![9,7,8]);
}

#[test]
fn tast_2() {
    assert_eq!(Solution::partition_labels("eccbbbbdec".to_string()), vec![10]);
}
