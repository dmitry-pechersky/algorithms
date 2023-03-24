struct Solution { }

impl Solution {
    pub fn maximal_network_rank(n: i32, roads: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let mut adj_matrix = vec![vec![false; n]; n];
        let mut cnts = vec![0; n];
        let mut max_value = 0;

        for (v, u) in roads.into_iter().map(|v| if v[0] < v[1] { (v[0], v[1]) } else { (v[1], v[0]) } ) {
            cnts[v as usize] += 1;
            cnts[u as usize] += 1;
            adj_matrix[v as usize][u as usize] = true;
        }
        for v in 0..n - 1 {
            for u in v + 1..n {
                max_value = max_value.max(cnts[v] + cnts[u] + if adj_matrix[v][u] { - 1 } else { 0 });
            }
        }
        max_value
    }
}

#[test]
fn test_1() {
    assert_eq!(4, Solution::maximal_network_rank(4, [[0,1],[0,3],[1,2],[1,3]].iter().map(|a| a.to_vec()).collect()));
    assert_eq!(5, Solution::maximal_network_rank(5, [[0,1],[0,3],[1,2],[1,3],[2,3],[2,4]].iter().map(|a| a.to_vec()).collect()));
    assert_eq!(5, Solution::maximal_network_rank(8, [[0,1],[1,2],[2,3],[2,4],[5,6],[5,7]].iter().map(|a| a.to_vec()).collect()));
}
