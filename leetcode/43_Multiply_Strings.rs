struct Solution {}

impl Solution {
    pub fn multiply(num1: String, num2: String) -> String {
        let num1 = num1.as_bytes().iter().rev().map(|v| v - 48).collect::<Vec<u8>>();
        let num2 = num2.as_bytes().iter().rev().map(|v| v - 48).collect::<Vec<u8>>();
        let (n1, n2) = (num1.len(), num2.len());
        let mut num3 = vec![0u8; n1 + n2];
        for i in 0..n1 {
            let mut trans = 0u8;
            for j in 0..n2 {
                let k = i + j;
                let tmp = num3[k] + num1[i] * num2[j] + trans;
                num3[k] = tmp % 10;
                trans = tmp / 10;
            }
            if trans > 0 {
                num3[i + n2] = trans;
            }
        }
        while num3.len() > 1 && num3[num3.len() - 1] == 0 {
            num3.pop();
        }
        String::from_utf8(num3.iter().rev().map(|v|  v + 48).collect()).unwrap()
    }
}

#[cfg(test)]
mod multiply_test {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::multiply("2".to_string(), "3".to_string()), "6".to_string());
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::multiply("123".to_string(), "456".to_string()), "56088".to_string());
    }
}