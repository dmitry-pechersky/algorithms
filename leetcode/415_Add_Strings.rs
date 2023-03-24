struct  Solution {}

impl Solution {
    pub fn add_strings(num1: String, num2: String) -> String {
        let num1 = num1.as_bytes().iter().rev().map(|v| v - b'0').collect::<Vec<_>>();
        let num2 = num2.as_bytes().iter().rev().map(|v| v - b'0').collect::<Vec<_>>();
        let (n1, n2) = (num1.len(), num2.len());
        let mut res = vec![];
        let mut transfer = 0;
        for i in 0..n1.min(n2) {
            let value = num1[i] + num2[i] + transfer;
            res.push(value % 10);
            transfer = value / 10;
        }
        for i in n1.min(n2)..n1 {
            let value = num1[i] + transfer;
            res.push(value % 10);
            transfer = value / 10;
        }
        for i in n1.min(n2)..n2 {
            let value = num2[i] + transfer;
            res.push(value % 10);
            transfer = value / 10;
        }
        if transfer > 0 {
            res.push(transfer);
        }
        res.iter().rev().map(|v| (v + b'0') as char).collect()
    }
}

#[test]
fn test_1() {
    assert_eq!(Solution::add_strings("11".to_string(), "123".to_string()), "134".to_string());
}

#[test]
fn test_2() {
    assert_eq!(Solution::add_strings("456".to_string(), "77".to_string()), "533".to_string());
}

#[test]
fn test_3() {
    assert_eq!(Solution::add_strings("0".to_string(), "0".to_string()), "0".to_string());
}
