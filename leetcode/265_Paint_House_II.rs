struct Solution {}

impl Solution {
    pub fn min_cost_ii(costs: Vec<Vec<i32>>) -> i32 {
        let k = costs[0].len();
        let mut cur = [0;20];
        for cost in costs.iter() {
            let (mut min_0, mut min_1) = (i32::MAX, i32::MAX);
            for i in 0..k {
                if cur[i] < min_0 {
                    min_1 = min_0;
                    min_0 = cur[i];
                } else if cur[i] < min_1 {
                    min_1 = cur[i];
                }
            }
            for i in 0..k {
                let min = if cur[i] == min_0 { min_1 } else { min_0 };
                cur[i] = cost[i] + min;
            }
        }
        *cur[0..k].iter().min().unwrap()
    }
}

#[test]
fn test_1() {
    assert_eq!(5, Solution::min_cost_ii(vec![vec![1,5,3],vec![2,9,4]]));
}

#[test]
fn test_2() {
    assert_eq!(5, Solution::min_cost_ii(vec![vec![1,3],vec![2,4]]));
}
