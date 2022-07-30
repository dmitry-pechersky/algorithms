struct Solution {}

impl Solution {
    pub fn is_bipartite(graph: Vec<Vec<i32>>) -> bool {
        let n = graph.len();
        let mut colours = vec![None; n];
        let mut stack = Vec::with_capacity(n);
        for i in 0..n {
            if colours[i].is_none() {
                stack.push(i);
                colours[i] = Some(true);
                while let Some(v) = stack.pop() {
                    for u in graph[v].iter().map(|x| *x as usize) {
                        if colours[u].is_none() {
                            stack.push(u);
                            colours[u] = colours[v].map(|x| !x);
                        } else if colours[u] == colours[v] { 
                            return false;
                        }
                    }
                }
            }
        }
        true
    }
}

#[test]
fn test_1() {
    assert!(! Solution::is_bipartite(vec![vec![1,2,3], vec![0,2], vec![0,1,3], vec![0,2]]));
}

#[test]
fn test_2() {
    assert!(Solution::is_bipartite(vec![vec![1,3], vec![0,2], vec![1,3], vec![0,2]]));
}

