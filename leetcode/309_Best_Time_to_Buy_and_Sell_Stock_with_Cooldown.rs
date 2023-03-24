struct Solution {}

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let (mut balance_with, mut balance_without, mut balance_cool) = (- prices[0], 0, 0);
        for price in prices.iter().skip(1) {
            let tmp_balance_with = balance_with.max(balance_without - price);
            let tmp_balance_without = balance_without.max(balance_cool);
            let tmp_balance_cool = balance_with + price;
            balance_with = tmp_balance_with;
            balance_without = tmp_balance_without;
            balance_cool = tmp_balance_cool;
        }
        balance_cool.max(balance_without)
    }
}

#[test]
fn test_1() {
    assert_eq!(3, Solution::max_profit(vec![1,2,3,0,2]));
}

#[test]
fn test_2() {
    assert_eq!(0, Solution::max_profit(vec![1]));
}
