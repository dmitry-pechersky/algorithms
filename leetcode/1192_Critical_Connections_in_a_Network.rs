struct Solution {}

impl Solution{
    pub fn critical_connections(n: i32, connections: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        fn rec(v: usize, depth: i32, w: usize, adj_list: &Vec<Vec<usize>>, depths: &mut Vec<i32>, bridges: &mut Vec<Vec<i32>>) {
            depths[v] = depth;
            for &u in adj_list[v].iter() {
                if u != w {
                    if depths[u] == 0 {
                        rec(u, depth + 1, v, adj_list, depths, bridges);
                        if depths[u] == depth + 1 {
                            bridges.push(vec![v as i32, u as i32]);
                        }
                    }
                    if depths[u] < depths[v] {
                        depths[v] = depths[u];
                    }
                }
            }
        }

        let mut bridges = vec![];
        let mut adj_list: Vec<Vec<usize>> = vec![vec![]; n as usize];
        for (v, u) in  connections.into_iter().map(|connection| (connection[0] as usize, connection[1] as usize)) {
            adj_list[v].push(u);
            adj_list[u].push(v);
        }
        let mut depths = vec![0; n as usize];
        rec(0, 1, n as usize, &adj_list, &mut depths, &mut bridges);
        bridges
    }
}

#[test]
fn test_1() {
    assert_eq!(
        vec![vec![1,3]],
        Solution::critical_connections(4, vec![vec![0,1],vec![1,2],vec![2,0],vec![1,3]])
    );
}

#[test]
fn test_2() {
    assert_eq!(
        vec![vec![0,1]],
        Solution::critical_connections(2, vec![vec![0,1]])
    );
}
