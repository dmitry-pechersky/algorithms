struct Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let n = prices.len();
        let mut dp = vec![0; n];
        let mut min_price = prices[0];
        for (i, &price) in prices.iter().enumerate().skip(1) {
            dp[i] = dp[i - 1].max(price - min_price);
            min_price = min_price.min(price);
        }
        let mut max_price = prices[n - 1];
        let mut max_right_profit = 0;
        let mut max_profit = 0;
        for (i, &price) in prices.iter().enumerate().rev().skip(1) {
            max_right_profit = max_right_profit.max(max_price - price);
            max_price = max_price.max(price);
            let max_left_profit = if i == 0 { 0 } else { dp[i - 1]};
            max_profit = max_profit.max(max_left_profit + max_right_profit);
        }
        max_profit
    }
}

#[test]
fn test_max_profit() {
    assert_eq!(6, Solution::max_profit(vec![3,3,5,0,0,3,1,4]));
    assert_eq!(4, Solution::max_profit(vec![1,2,3,4,5]));
    assert_eq!(0, Solution::max_profit(vec![7,6,4,3,1]));
}