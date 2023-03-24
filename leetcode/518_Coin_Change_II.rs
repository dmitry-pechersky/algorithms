struct Solution {}

impl Solution {
    pub fn change(amount: i32, coins: Vec<i32>) -> i32 {
        let mut dp = vec![0; amount as usize  + 1];
        dp[0] = 1;
        for coin in coins {
            for i in 0..=amount - coin {
                dp[i as usize + coin as usize] += dp[i as usize];
            }    
        }
        dp[amount as usize]    
    }
}


#[test]
fn test_1() {
    assert_eq!(4, Solution::change(5, vec![1,2,5]));
    assert_eq!(0, Solution::change(3, vec![2]));
    assert_eq!(1, Solution::change(10, vec![10]));
}
