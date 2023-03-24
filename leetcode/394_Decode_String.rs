struct Solution {}

impl Solution {
    pub fn decode_string(s: String) -> String {
        fn number(s: &Vec<char>, i: &mut usize) -> u32 {
            let mut res = 0;
            while s[*i].is_digit(10) {
                res = res * 10 + s[*i].to_digit(10).unwrap();
                *i += 1;
            }
            res
        }

        fn text(s: &Vec<char>, i: &mut usize, res: &mut Vec<char>) {
            while *i < s.len() &&  s[*i].is_ascii_alphabetic() {
                res.push(s[*i]);
                *i += 1;
            }
        }

        fn rec(s: &Vec<char>, i: &mut usize, res: &mut Vec<char>) {
            while *i < s.len() && s[*i] != ']' {
                if s[*i].is_digit(10) {
                    let count = number(s, i);
                    let start_sub_res = res.len();
                    *i += 1;
                    rec(s, i, res);
                    *i += 1;
                    let end_sub_res = res.len();
                    for _ in 0..count - 1 {
                        for j in start_sub_res..end_sub_res {
                            res.push(res[j]);
                        }
                    }
                } else {
                    text(s, i, res)
                }
            }
        }

        let s = s.chars().collect::<Vec<_>>();
        let mut res: Vec<char>= vec![];        
        rec(&s, &mut 0, &mut res);
        res.into_iter().collect()
    }
}

#[test]
fn test_1() {
    assert_eq!(Solution::decode_string("3[a]2[bc]".to_string()), "aaabcbc".to_string());
    assert_eq!(Solution::decode_string("3[a2[c]]".to_string()), "accaccacc".to_string());
    assert_eq!(Solution::decode_string("2[abc]3[cd]ef".to_string()), "abcabccdcdcdef".to_string());
}
