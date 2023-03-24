struct Solution { }

impl Solution {
    pub fn min_reorder(n: i32, connections: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let mut adj_list = vec![vec![]; n];
        let mut rev_adj_list = vec![vec![]; n];
        let mut stack = vec![0];
        let mut visited = vec![false; n];
        visited[0] = true;
        let mut cnt = 0;

        for (v, u) in connections.into_iter().map(|v| (v[0] as usize, v[1] as usize)) {
            adj_list[v].push(u);
            rev_adj_list[u].push(v);
        }

        while let Some(v) = stack.pop() {
            for &u in &rev_adj_list[v] {
                if !visited[u] {
                    visited[u] = true;
                    stack.push(u);
                }
            }
            for &u in &adj_list[v] {
                if !visited[u] {
                    visited[u] = true;
                    stack.push(u);
                    cnt += 1;
                }
            }
        }
        cnt
    }
}

#[test]
fn test_1() {
    assert_eq!(3, Solution::min_reorder(6, [[0,1],[1,3],[2,3],[4,0],[4,5]].iter().map(|v| v.to_vec()).collect()));
    assert_eq!(2, Solution::min_reorder(5, [[1,0],[1,2],[3,2],[3,4]].iter().map(|v| v.to_vec()).collect()));
    assert_eq!(0, Solution::min_reorder(3, [[1,0],[2,0]].iter().map(|v| v.to_vec()).collect()));
}
