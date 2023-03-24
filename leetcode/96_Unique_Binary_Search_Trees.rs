struct Solution {}

impl Solution {
    pub fn num_trees(n: i32) -> i32 {
        let n = n as usize;
        let mut dp = vec![1; n + 1];
        for i in 2..=n {
            let mut cnt = 0;
            for j in 0..i {
                cnt += dp[j] * dp[i - j - 1];
            }
            dp[i] = cnt;
        }
        dp[n]
    }
}

#[test]
fn test_1() {
    assert_eq!(5, Solution::num_trees(3));
    assert_eq!(1, Solution::num_trees(1));
}
