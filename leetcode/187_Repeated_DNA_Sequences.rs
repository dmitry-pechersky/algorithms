struct Solution {}

impl Solution {
    pub fn find_repeated_dna_sequences(s: String) -> Vec<String> {
        let s = s.into_bytes();
        let n = s.len();
        let mut res = vec![];
        let mut cur_hash = 0u32;
        let mask = !((! 0u32) << 20);
        let mut cnts = [0u8; 2usize.pow(20)];
        for i in 0..n {
            let ch = match s[i] {b'A' => 0, b'C' => 1, b'G' => 2, b'T' => 3, _ => unreachable!()};
            cur_hash = (cur_hash << 2 | ch) & mask;
            if i >= 9 {
                cnts[cur_hash as usize] = cnts[cur_hash as usize].saturating_add(1);
                if cnts[cur_hash as usize] == 2 {
                    res.push(String::from_utf8(s[i - 9..i + 1].to_vec()).unwrap());
                }
            }
        }
        res
    }
}

#[test]
fn test_1() {
    let mut res = Solution::find_repeated_dna_sequences("AAAAACCCCCAAAAACCCCCCAAAAAGGGTTT".to_string());
    res.sort();
    assert_eq!(
        vec!["AAAAACCCCC".to_string(), "CCCCCAAAAA".to_string()], 
        res
    );
}

#[test]
fn test_2() {
    assert_eq!(
        vec!["AAAAAAAAAA".to_string()],
        Solution::find_repeated_dna_sequences("AAAAAAAAAAAAA".to_string())
    );
    
}
