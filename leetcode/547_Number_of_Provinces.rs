struct  Solution {}

impl Solution {
    pub fn find_circle_num(is_connected: Vec<Vec<i32>>) -> i32 {
        fn dfs(v: usize, n: usize, adj_matrix: &Vec<Vec<i32>>, visited: &mut Vec<bool>) {
            for u in 0..n {
                if adj_matrix[v][u] == 1 && !visited[u] {
                    visited[u] = true;
                    dfs(u, n, adj_matrix, visited);
                }
            }
        }

        let n = is_connected.len();
        let mut visited = vec![false; n];
        let mut cnt = 0;
        for v in 0..n {
            if ! visited[v] {
                cnt += 1;
                visited[v] = true;
                dfs(v, n, &is_connected, &mut visited);
            }
        }
        cnt
    }
}

#[test]
fn test_1() {
    assert_eq!(2, Solution::find_circle_num(vec![vec![1,1,0],vec![1,1,0],vec![0,0,1]]));
}

#[test]
fn test_2() {
    assert_eq!(3, Solution::find_circle_num(vec![vec![1,0,0],vec![0,1,0],vec![0,0,1]]));
}
