struct Solution {}

impl Solution {
    pub fn num_of_minutes(n: i32, head_id: i32, manager: Vec<i32>, inform_time: Vec<i32>) -> i32 {
        let mut max_time = 0;
        let mut dp = vec![None; n as usize];
        dp[head_id as usize] = Some(0);

        for i in 0..n {
            let mut time = 0;

            let mut employee = i;
            while dp[employee as usize].is_none() {
                employee = manager[employee as usize];
                time += inform_time[employee as usize];
            }
            time += dp[employee as usize].unwrap();
            max_time = max_time.max(time);

            let mut employee = i;
            while dp[employee as usize].is_none() {
                dp[employee as usize] = Some(time);
                employee = manager[employee as usize];
                time -= inform_time[employee as usize];
            }
        }
        max_time
    }
}

#[test]
fn test_1() {
    assert_eq!(0, Solution::num_of_minutes(1, 0, vec![-1], vec![0]));
    assert_eq!(1, Solution::num_of_minutes(6, 2, vec![2,2,-1,2,2,2], vec![0,0,1,0,0,0]));
}
