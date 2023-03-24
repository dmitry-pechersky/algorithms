use std::collections::{VecDeque, HashSet};

struct Solution { }

impl Solution {
    pub fn shortest_path_length(graph: Vec<Vec<i32>>) -> i32 {
        let n = graph.len();
        let target = u16::MAX >> (16 - n);
        let mut queue: VecDeque<(u8, u16, u8)> = (0..n as u8).map(|v| (v, 1 << v, 0)).collect();
        let mut visited: HashSet<(u8, u16)> = HashSet::new();


        while let Some((v, state, length)) = queue.pop_front() {
            if ! visited.contains(&(v, state)) {
                if state == target {
                    return length as i32;
                }
                visited.insert((v, state));
                for &u in &graph[v as usize] {
                    queue.push_back((u as u8, state | (1 << u), length + 1));
                }
            }
        }
        unreachable!()
    }
}

#[test]
fn test_1() {
    assert_eq!(4, Solution::shortest_path_length(vec![vec![1,2,3],vec![0],vec![0],vec![0]]));
    assert_eq!(4, Solution::shortest_path_length(vec![vec![1],vec![0,2,4],vec![1,3,4],vec![2],vec![1,2]]));
}