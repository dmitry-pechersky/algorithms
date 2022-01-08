struct Solution {}

impl Solution {
    pub fn can_reach(arr: Vec<i32>, start: i32) -> bool {
        let n = arr.len();
        let mut stack = vec![start];
        let mut visited = vec![false; n];
        while let Some(node) = stack.pop() {
            if arr[node as usize] == 0 {
                return true
            }
            if ! visited[node as usize] {
                visited[node as usize] = true;
                if node >= arr[node as usize] {
                    stack.push(node - arr[node as usize]);
                }
                if node + arr[node as usize] < n as i32 {
                    stack.push(node + arr[node as usize]);
                }
            }
        }
        false
    }
}

mod can_reach_test {
    use super::*;

    #[test]
    fn test_1() {
        assert!(Solution::can_reach(vec![4,2,3,0,3,1,2], 5));
    }

    #[test]
    fn test_2() {
        assert!(Solution::can_reach(vec![4,2,3,0,3,1,2], 0));
    }

    #[test]
    fn test_3() {
        assert!(!Solution::can_reach(vec![3,0,2,1,2], 2));
    }

}