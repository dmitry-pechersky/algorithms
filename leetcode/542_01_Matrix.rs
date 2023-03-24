use std::collections::VecDeque;

struct Solution { }

impl Solution {
    pub fn update_matrix(mat: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let (n, m) = (mat.len(), mat[0].len());
        let mut mat = mat;
        let mut distances = vec![vec![0; m]; n];
        let mut queue = VecDeque::new();
        for i in 0..n {
            for j in 0..m {
                if mat[i][j] == 0 {
                    queue.push_back((i, j, 0));
                }
            }
        }
        while let Some((i, j, distance)) = queue.pop_front() {
            for (next_i, next_j) in [(1, 0), (0, 1), (-1, 0), (0, -1)].iter()
                .map(|(di, dj)| (i as i32 + di, j as i32 + dj))
                .filter(|&(i, j)| i >= 0 && i < n as i32 && j >= 0 && j < m as i32)
                .map(|(i, j)| (i as usize, j as usize))
            {
                if mat[next_i][next_j] == 1 {
                    mat[next_i][next_j] = 0;
                    distances[next_i][next_j] = distance + 1;
                    queue.push_back((next_i, next_j, distance + 1));
                }
            }
        }
        distances
    }
}

#[test]
fn test_1() {
    assert_eq!(
        vec![vec![0,0,0],vec![0,1,0],vec![0,0,0]],
        Solution::update_matrix(vec![vec![0,0,0],vec![0,1,0],vec![0,0,0]])
    );
}

#[test]
fn test_2() {
    assert_eq!(
        vec![vec![0,0,0],vec![0,1,0],vec![1,2,1]],
        Solution::update_matrix(vec![vec![0,0,0],vec![0,1,0],vec![1,1,1]])
    );
}
