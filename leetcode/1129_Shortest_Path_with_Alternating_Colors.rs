use std::collections::VecDeque;

struct Solution {}

impl Solution {
    pub fn shortest_alternating_paths(n: i32, red_edges: Vec<Vec<i32>>, blue_edges: Vec<Vec<i32>>) -> Vec<i32> {
        fn edges_to_adj_list(n: usize, edges: &Vec<Vec<i32>>) -> Vec<Vec<usize>> {
            let mut adj_list = vec![vec![]; n];
            for (v, u) in edges.iter().map(|v| (v[0] as usize, v[1] as usize)) {
                adj_list[v].push(u);
            }
            adj_list
        }

        let n = n as usize;
        let adj_lists = [edges_to_adj_list(n, &blue_edges), edges_to_adj_list(n, &red_edges)];
        let mut lengths = [vec![None; n], vec![None; n]];
        let mut queue: VecDeque<(usize, i32, u8)> = VecDeque::new();
        queue.push_back((0, 0, 0));
        queue.push_back((0, 0, 1));
        while let Some((v, length, color)) = queue.pop_front() {
            if lengths[color as usize][v].is_none() {
                lengths[color as usize][v] = Some(length);
                let next_color = (color + 1) % 2;
                for &u in adj_lists[next_color as usize][v].iter() {
                    queue.push_back((u, length + 1, next_color));
                }
            }
        }
        (0..n).into_iter()
            .map(
                |i|
                match (lengths[0][i], lengths[1][i]) {
                    (Some(length0), Some(length1)) => length0.min(length1),
                    (Some(length0), None) => length0,
                    (None, Some(length1)) => length1,
                    (None, None) => -1, 
                }
            ).collect()
    }
}

#[test]
fn test_1() {
    assert_eq!(vec![0,1,-1], Solution::shortest_alternating_paths(3, vec![vec![0,1],vec![1,2]], Vec::<Vec<i32>>::new()));
    assert_eq!(vec![0,1,-1], Solution::shortest_alternating_paths(3, vec![vec![0,1]], vec![vec![2,1]]));
}