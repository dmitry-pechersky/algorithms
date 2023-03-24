struct Solution { }

impl Solution {
    pub fn minimum_delete_sum(s1: String, s2: String) -> i32 {
        let (s1, s2) = (s1.as_bytes(), s2.as_bytes());
        let (n1, n2) = (s1.len(), s2.len());
        let mut dp_1 = vec![0; n1 + 1];
        let mut dp_2 = vec![0; n1 + 1];
        for i in 0..n1 {
            dp_1[i + 1] = dp_1[i] + s1[i] as i32;
        }
        for i2 in 0..n2 {
            dp_2[0] = dp_1[0] + s2[i2] as i32;
            for i1 in 0..n1 {
                dp_2[i1 + 1] = (dp_1[i1 + 1] + s2[i2] as i32).min(dp_2[i1] + s1[i1] as i32);
                if s1[i1] == s2[i2] && dp_1[i1] < dp_2[i1 + 1] {
                    dp_2[i1 + 1] = dp_1[i1];
                }
            }
            let tmp = dp_1;
            dp_1 = dp_2;
            dp_2 = tmp;
        }
        dp_1[n1]
    }
}

#[test]
fn test_1() {
    assert_eq!(231, Solution::minimum_delete_sum("sea".to_string(), "eat".to_string()));
}

#[test]
fn test_2() {
    assert_eq!(403, Solution::minimum_delete_sum("delete".to_string(), "leet".to_string()));
}