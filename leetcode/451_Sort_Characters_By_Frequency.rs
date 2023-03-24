struct Solution {}

impl Solution {
    pub fn frequency_sort(s: String) -> String {
        let mut cnts = [0; 256];
        for &ch in s.as_bytes() {
            cnts[ch as usize] += 1;
        }
        let mut cnts = cnts.iter()
            .enumerate()
            .filter(|(_, &cnt)| cnt > 0)
            .map(|(ch, cnt)| (*cnt, ch as u8 as char))
            .collect::<Vec<_>>();
        cnts.sort();
        let mut res = String::with_capacity(s.len());
        for (cnt, ch) in cnts.into_iter().rev() {
            for _ in 0..cnt {
                res.push(ch);
            }
        }
        res
    }
}

#[test]
fn test_1(){
    assert!(
        ["eert".to_string(), "eetr".to_string()].contains(&Solution::frequency_sort("tree".to_string()))
    );
}

#[test]
fn test_2(){
    assert!(
        ["aaaccc".to_string(), "cccaaa".to_string()].contains(&Solution::frequency_sort("cccaaa".to_string()))
    );
}

#[test]
fn test_3(){
    assert!(
        ["bbaA".to_string(), "bbAa".to_string()].contains(&Solution::frequency_sort("Aabb".to_string()))
    );
}
