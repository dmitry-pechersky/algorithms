struct Solution {}

impl Solution {
    pub fn min_cost(costs: Vec<Vec<i32>>) -> i32 {
        let mut res = [0, 0, 0];
        for cost  in costs.iter() {
            res = [
                cost[0] + res[1].min(res[2]),
                cost[1] + res[0].min(res[2]),
                cost[2] + res[0].min(res[1])
            ];
        }
        *res.iter().min().unwrap()
    }
}

#[test]
fn test_1() {
    assert_eq!(10, Solution::min_cost(vec![vec![17,2,17],vec![16,16,5],vec![14,3,19]]));
}

#[test]
fn test_2() {
    assert_eq!(2, Solution::min_cost(vec![vec![7,6,2]]));
}