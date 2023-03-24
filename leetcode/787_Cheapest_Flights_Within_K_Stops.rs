use std::collections::{BinaryHeap};
use core::cmp::Reverse;

fn edges_to_adj_list(n: usize, edges: &Vec<Vec<i32>>) -> Vec<Vec<(usize, i32)>> {
    let mut adj_list = vec![vec![]; n];
    for (v, u, weight) in edges.iter().map(|v| (v[0] as usize, v[1] as usize, v[2] as i32)) {
        adj_list[v].push((u, weight));
    }
    adj_list
} 

struct Solution {}

impl Solution {
    pub fn find_cheapest_price(n: i32, flights: Vec<Vec<i32>>, src: i32, dst: i32, k: i32) -> i32 {
        let adj_list = edges_to_adj_list(n as usize, &flights);
        let mut heap = BinaryHeap::from([(Reverse(0), Reverse(0), src as usize)]);
        let mut weights = vec![i32::MAX; n as usize];
        let mut cnts = vec![i32::MAX; n as usize];
        while let Some((Reverse(v_weight), Reverse(v_cnt), v)) = heap.pop() {
            if v == dst as usize {
                return v_weight;
            }
            if v_cnt <= k {
                for &(u, weight) in &adj_list[v] {
                    let (u_weight, u_cnt) = (v_weight + weight, v_cnt + 1);
                    if u_weight < weights[u] || u_cnt < cnts[u] {
                        heap.push((Reverse(u_weight), Reverse(u_cnt), u));
                        if u_weight <= weights[u] && u_cnt <= cnts[u] {
                            weights[u] = u_weight;
                            cnts[u] = u_cnt;
                        }
                    }
                }    
            }
        }
        -1
    }
}

#[test]
fn test_1() {
    assert_eq!(700, Solution::find_cheapest_price(4, vec![vec![0,1,100], vec![1,2,100], vec![2,0,100], vec![1,3,600], vec![2,3,200]], 0, 3, 1));
    assert_eq!(200, Solution::find_cheapest_price(3, vec![vec![0,1,100], vec![1,2,100], vec![0,2,500]], 0, 2, 1));
    assert_eq!(500, Solution::find_cheapest_price(3, vec![vec![0,1,100], vec![1,2,100], vec![0,2,500]], 0, 2, 0));
}
