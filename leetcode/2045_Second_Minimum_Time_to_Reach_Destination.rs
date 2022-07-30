use std::collections::VecDeque;

impl Solution {
    pub fn second_minimum(n: i32, edges: Vec<Vec<i32>>, time: i32, change: i32) -> i32 {
        let n = n as usize;
        let mut adj_list: Vec<Vec<usize>> = vec![vec![]; n];
        for (v, u) in edges.into_iter().map(|edge| (edge[0] as usize - 1, edge[1] as usize - 1)) {
            adj_list[v].push(u);
            adj_list[u].push(v);
        }
        let mut queue: VecDeque<(i32, usize)>  = VecDeque::from([(0, 0)]);
        let mut visited: Vec<[Option<i32>; 2]>= vec![[None, None]; n];
        visited[0][0] = Some(0); 
        while let Some((cost, v)) = queue.pop_front() {
            if v == n - 1 {
                if let Some(cost_1) = visited[v][1] {
                    return cost_1;
                }
            }
            let wait_time = if cost % (2 * change) < change { 0 } else { 2 * change -  cost % (2 * change) };
            let next_cost = cost + time + wait_time;
            for &u in adj_list[v].iter() {
                match (visited[u][0], visited[u][1]) {
                    (None, None) => { visited[u][0] = Some(next_cost); },
                    (Some(cost_0), None) if cost_0 < next_cost => { visited[u][1] = Some(next_cost); },
                    (_, _) => { continue; },
                };
                queue.push_back((next_cost, u));
            }
        }
        unreachable!()
    }
}
