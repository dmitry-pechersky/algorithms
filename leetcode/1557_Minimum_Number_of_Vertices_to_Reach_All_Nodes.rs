struct Solution {}

impl Solution {
    pub fn find_smallest_set_of_vertices(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        let mut has_in_edges = vec![false; n as usize];
        edges.iter().for_each(|v| { has_in_edges[v[1] as usize] = true });
        (0..n).filter(|i| !has_in_edges[*i as usize]).collect()
    }
}

#[test]
fn test_1() {
    assert_eq!(vec![0,3],  Solution::find_smallest_set_of_vertices(6, vec![vec![0,1],vec![0,2],vec![2,5],vec![3,4],vec![4,2]]));
}

#[test]
fn test_2() {
    assert_eq!(vec![0,2,3],  Solution::find_smallest_set_of_vertices(5, vec![vec![0,1],vec![2,1],vec![3,1],vec![1,4],vec![2,4]]));
}
