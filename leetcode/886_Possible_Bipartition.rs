struct Solution {}

impl Solution {
    pub fn possible_bipartition(n: i32, dislikes: Vec<Vec<i32>>) -> bool {
        let mut adj_list = vec![vec![]; n as usize];
        for (v, u) in  dislikes.iter().map(|v| (v[0] as usize - 1, v[1] as usize - 1)) {
            adj_list[v].push(u);
            adj_list[u].push(v);
        }
        let mut partitions = vec![None; n as usize];
        let mut stack = vec![];
        for v in 0..n as usize {
            if partitions[v].is_none() {
                partitions[v] = Some(true);
                stack.push(v);
                while let Some(v) = stack.pop() {
                    let v_partition = partitions[v].unwrap();
                    for &u in adj_list[v].iter() {
                        if let Some(u_partition)  = partitions[u] {
                            if u_partition == v_partition {
                                return false
                            }
                        } else {
                            partitions[u] = Some(!v_partition);
                            stack.push(u);
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
    assert!(
        Solution::possible_bipartition(4, vec![vec![1,2],vec![1,3],vec![2,4]])
    )
}

#[test]
fn test_2() {
    assert!(
        !Solution::possible_bipartition(4, vec![vec![1,2],vec![1,3],vec![2,3]])
    )
}

#[test]
fn test_3() {
    assert!(
        !Solution::possible_bipartition(5, vec![vec![1,2],vec![2,3],vec![3,4],vec![4,5],vec![1,5]])
    )
}
