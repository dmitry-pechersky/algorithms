impl Solution {
    pub fn integer_break(n: i32) -> i32 {
        if n == 2 {
            1
        } else if n == 3 {
            2
        } 
        else {
            let mut n = n;
            let mut product =1;
            while n > 0 {
                if n == 2 || n == 4 {
                    n -= 2;
                    product *= 2;
                } else {
                    n -= 3;
                    product *= 3;
                }
            }
            product
        }
    }
}

#[test]
fn test_1() {
    assert_eq!(1, Solution::integer_break(2));
    assert_eq!(36, Solution::integer_break(10));
}