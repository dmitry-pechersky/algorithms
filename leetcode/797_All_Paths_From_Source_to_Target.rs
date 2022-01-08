struct Solution {}

impl Solution {
    pub fn all_paths_source_target(graph: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut paths = vec![];
        let mut path = vec![];
        fn _rec(graph: &Vec<Vec<i32>>, node: i32, path: &mut Vec<i32>, paths: &mut Vec<Vec<i32>>) {
            path.push(node);
            if node as usize == graph.len()  - 1 {
                paths.push(path.clone());
            } else {
                for successor in &graph[node as usize] {
                    _rec(graph, *successor, path, paths);
                }    
            }
            path.pop();
        }
        _rec(&graph, 0, &mut path, &mut paths);
        paths
    }
}

mod all_paths_source_target_test {
    use super::*;

    #[test]
    fn test_1() {
        let mut res =  Solution::all_paths_source_target(vec![vec![1,2], vec![3], vec![3], vec![]]);
        res.sort();
        assert_eq!(res, vec![vec![0,1,3], vec![0,2,3]]);
    }

    #[test]
    fn test_2() {
        let mut res =  Solution::all_paths_source_target(vec![vec![4,3,1], vec![3,2,4], vec![3], vec![4], vec![]]);
        res.sort();
        let mut target = vec![vec![0,4], vec![0,3,4], vec![0,1,3,4], vec![0,1,2,3,4], vec![0,1,4]];
        target.sort();

        assert_eq!(res, target);
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::all_paths_source_target(vec![vec![1], vec![]]), vec![vec![0,1]]);
    }

    #[test]
    fn test_4() {
        let mut res =  Solution::all_paths_source_target(vec![vec![1,2,3], vec![2], vec![3], vec![]]);
        res.sort();
        assert_eq!(res, vec![vec![0,1,2,3], vec![0,2,3], vec![0,3]]);
    }

    #[test]
    fn test_5() {
        let mut res =  Solution::all_paths_source_target(vec![vec![1,3], vec![2], vec![3], vec![]]);
        res.sort();
        assert_eq!(res, vec![vec![0,1,2,3], vec![0,3]]);
    }

}
