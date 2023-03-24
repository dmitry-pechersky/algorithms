struct Solution {}

impl Solution {
    pub fn eventual_safe_nodes(graph: Vec<Vec<i32>>) -> Vec<i32> {
        fn dfs(v: usize, graph: &Vec<Vec<i32>>, in_stack: &mut Vec<bool>, is_safe: &mut Vec<Option<bool>>) -> bool {
            if is_safe[v].is_none() {
                if in_stack[v] {
                    is_safe[v] = Some(false);                    
                } else {
                    let mut safe = true;
                    in_stack[v] = true;
                    for u in graph[v].iter().map(|&a| a as usize) {
                        if ! dfs(u, graph, in_stack, is_safe) {
                            safe = false;
                            break;
                        }
                    }
                    in_stack[v] = false;
                    is_safe[v] = Some(safe);
                }
            }
            is_safe[v].unwrap()
        }

        let n = graph.len();
        let mut in_stack = vec![false; n];
        let mut is_safe = vec![None; n];
        let mut result = vec![];
        
        for v in 0..n as usize {
            if dfs(v, &graph, &mut in_stack, &mut is_safe) {
                result.push(v as i32);
            }
        }

        result
    }
}

#[test]
fn test_1() {
    assert_eq!(Vec::<i32>::new(), Solution::eventual_safe_nodes(vec![vec![1],vec![0]]));
    assert_eq!(vec![2,4,5,6], Solution::eventual_safe_nodes(vec![vec![1,2],vec![2,3],vec![5],vec![0],vec![5],vec![],vec![]]));
    assert_eq!(vec![4], Solution::eventual_safe_nodes(vec![vec![1,2,3,4],vec![1,2],vec![3,4],vec![0,4],vec![]]));
}
