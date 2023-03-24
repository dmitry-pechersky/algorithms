use std::collections::{VecDeque, HashSet};

struct  Solution { }

impl Solution {
    pub fn open_lock(deadends: Vec<String>, target: String) -> i32 {
        fn string_to_code(s: &String) -> u16 {
            let mut res: u16 = 0;
            for digit in s.as_bytes().iter().map(|&v| v - b'0') {
                res = res * 10 + digit as u16;
            }
            res
        }

        fn next_codes(code: u16, i: u32) -> [u16; 2] {
            let shift = 10u16.pow(i);
            let digit = (code / shift) % 10;
            [
                if digit < 9 { code + shift } else { code - 9 * shift },
                if digit > 0 { code - shift } else { code + 9 * shift }
            ]
        }

        let target = string_to_code(&target);
        let mut deadends: HashSet<u16> = deadends.into_iter().map(|s| string_to_code(&s)).collect();
        if !deadends.contains(&0) {
            let mut queue: VecDeque<(u16, i32)> = [(0, 0)].into();

            while let Some((code, cnt)) = queue.pop_front() {
                if code == target {
                    return cnt;
                }
                for i in 0..4 {
                    for next_code in next_codes(code, i) {
                        if !deadends.contains(&next_code) {
                            deadends.insert(next_code);
                            queue.push_back((next_code, cnt + 1));
                        }    
                    }
                }
            }    
        }
        -1 
    }
}

#[test]
fn test_1() {
    assert_eq!(
        6, 
        Solution::open_lock(
            ["0201","0101","0102","1212","2002"].iter().map(|s| s.to_string()).collect(), 
            "0202".to_string())
    );

    assert_eq!(
        1, 
        Solution::open_lock(
            vec!["8888".to_string()], 
            "0009".to_string())
    );

    assert_eq!(
        -1, 
        Solution::open_lock(
            ["8887","8889","8878","8898","8788","8988","7888","9888"].iter().map(|s| s.to_string()).collect(), 
            "8888".to_string())
    );

    assert_eq!(
        -1,
        Solution::open_lock(vec!["0000".to_string()], "8888".to_string())
    );
}