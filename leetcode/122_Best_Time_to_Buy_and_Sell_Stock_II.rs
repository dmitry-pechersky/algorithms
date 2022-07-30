struct Solution {}

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut profit = 0;
        for i in 1..prices.len() {
            if prices[i] > prices[i - 1]{
                profit += prices[i] - prices[i - 1] ;
            }
        }
        return profit;
    }
}

mod max_profit {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::max_profit(vec![7,1,5,3,6,4]), 7);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::max_profit(vec![1,2,3,4,5]), 4);
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::max_profit(vec![7,6,4,3,1]), 0);
    }

}
