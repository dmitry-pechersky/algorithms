use std::collections::VecDeque;
struct Solution {}

impl Solution {
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        if amount == 0 {
            return 0;
        }
        let coins = coins.into_iter()
            .filter_map(|coin| if coin <= amount { Some(coin as usize) } else { None })
            .collect::<Vec<_>>();
        let amount= amount as usize;

        let mut visited: Vec<Option<usize>> = vec![None; amount + 1];
        coins.iter().for_each(|&coin| visited[coin] = Some(1));
        let mut queue = coins.iter().map(|&coin| (1usize, coin)).collect::<VecDeque<_>>();
        while let Some((cnt, v)) = queue.pop_front() {
            if v == amount {
                return cnt as i32;
            }
            for &coin in coins.iter() {
                if coin <=  amount - v {
                    let u =  v + coin;
                    if visited[u].is_none() {
                        visited[u] = Some(cnt + 1);
                        queue.push_back((cnt + 1, u));
                    }
                } 
            }
        }
        -1
    }

    pub fn _coin_change_1(coins: Vec<i32>, amount: i32) -> i32 {
        if amount == 0 {
            return 0;
        }
        let coins = coins.into_iter()
            .filter_map(|coin| if coin <= amount { Some(coin as usize) } else { None })
            .collect::<Vec<_>>();
        let amount= amount as usize;

        let mut dp: Vec<Option<usize>> = vec![None; amount + 1];
        for &coin in coins.iter() {
            dp[coin] = Some(1);
        }
        for i in 1..=amount {
            if let Some(i_cnt) = dp[i] {
                for &coin in coins.iter().filter(|&&coin| coin <= amount - i) {
                    let next_amount = i + coin;
                    dp[next_amount] = dp[next_amount].map_or(Some(i_cnt + 1), |next_cnt| Some(next_cnt.min(i_cnt + 1)));
                }
            }
        }
        dp[amount].map_or(-1, |cnt| cnt as i32)
    }
}

#[test]
fn test_1() {
    assert_eq!(3, Solution::coin_change(vec![1,2,5], 11));
}

#[test]
fn test_2() {
    assert_eq!(-1, Solution::coin_change(vec![2], 3));
}
#[test]
fn test_3() {
    assert_eq!(0, Solution::coin_change(vec![1], 0));
}

#[test]
fn test_4() {
    assert_eq!(2, Solution::coin_change(vec![1,2147483647], 2));
}
