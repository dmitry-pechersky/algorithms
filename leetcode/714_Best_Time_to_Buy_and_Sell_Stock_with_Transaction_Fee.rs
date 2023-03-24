struct Solution {}

impl Solution {
    pub fn max_profit(prices: Vec<i32>, fee: i32) -> i32 {
        let (mut balance_0, mut balance_1) = (0, - prices[0] - fee);
        for price in prices.iter().skip(1) {
            let tmp_balance_0 = (balance_1 + price).max(balance_0);
            let tmp_balance_1 = (balance_0 - price - fee).max(balance_1);
            balance_0 = tmp_balance_0;
            balance_1 = tmp_balance_1;
        }
        balance_0
    }
}

#[test]
fn test_1() {
    assert_eq!(8, Solution::max_profit(vec![1,3,2,8,4,9], 2));
}

#[test]
fn test_2() {
    assert_eq!(6, Solution::max_profit(vec![1,3,7,5,10,3], 3));
}