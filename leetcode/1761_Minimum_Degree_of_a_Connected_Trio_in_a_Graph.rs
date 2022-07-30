struct Solution {}

impl Solution {
    pub fn min_trio_degree(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let mut adj_matrix = vec![vec![false; n]; n];
        let mut edge_cnts = vec![0; n];
        let mut min_edge_cnt = n * 3;
        for edge in edges {
            let (v, u) = (edge[0] as usize - 1, edge[1] as usize - 1);
            adj_matrix[v][u] = true;
            adj_matrix[u][v] = true;
            edge_cnts[v] += 1;
            edge_cnts[u] += 1;
        }
        for v in 0..n {
            for u in v + 1..n {
                for w in u + 1..n {
                    if adj_matrix[v][u] && adj_matrix[u][w] && adj_matrix[v][w] {
                        let edge_cnt = edge_cnts[v] + edge_cnts[u] + edge_cnts[w];
                        if edge_cnt < min_edge_cnt {
                            min_edge_cnt = edge_cnt;
                        }
                    }
                }
            }
        }
        if min_edge_cnt < n * 3 { min_edge_cnt as i32 - 6 } else { -1 }
    }
}

#[test]
fn test_1() {
    assert_eq!(3 , Solution::min_trio_degree(6, vec![vec![1,2], vec![1,3], vec![3,2], vec![4,1], vec![5,2], vec![3,6]]));
}

#[test]
fn test_2() {
    assert_eq!(0 , Solution::min_trio_degree(7, vec![vec![1,3], vec![4,1], vec![4,3], vec![2,5], vec![5,6], vec![6,7], vec![7,5], vec![2,6]]));
}

