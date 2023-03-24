struct Solution { }

impl Solution {
    pub fn remove_kdigits(num: String, k: i32) -> String {
        let mut k = k;
        let mut stack = vec![];
        for ch in num.chars() {
            while ! stack.is_empty() && k > 0 && *stack.last().unwrap() as u8 > ch as u8  {
                stack.pop();
                k -= 1;
            }
            if ! (ch == '0' && stack.is_empty()) {
                stack.push(ch);
            }
        }
        while k > 0 && !stack.is_empty() {
            stack.pop();
            k -= 1;
        }
        if stack.is_empty() { "0".to_string() } else { stack.into_iter().collect() }
    }
}

#[test]
fn test_1() {
    assert_eq!("1219".to_string(), Solution::remove_kdigits("1432219".to_string(), 3));
}

#[test]
fn test_2() {
    assert_eq!("200".to_string(), Solution::remove_kdigits("10200".to_string(), 1));
}

#[test]
fn test_3() {
    assert_eq!("0".to_string(), Solution::remove_kdigits("10".to_string(), 2));
}