use std::collections::VecDeque;

struct Solution { }

impl Solution {
    pub fn nearest_exit(maze: Vec<Vec<char>>, entrance: Vec<i32>) -> i32 {
        let (n, m) = (maze.len(), maze[0].len());
        let mut maze = maze;
        let mut queue = VecDeque::new();
        queue.push_back((entrance[0] as usize, entrance[1] as usize, 0));
        maze[entrance[0] as usize][entrance[1] as usize] = '+';

        while let Some((i, j, distance)) = queue.pop_front() {
            for (next_i, next_j) in [(1, 0), (0, 1), (-1, 0), (0, -1)].iter()
                .map(|(di, dj)| ( i as i32 + di, j as i32 + dj))
                .filter(|&(i, j)| i >= 0 && i < n as i32 && j >= 0 && j < m as i32)
                .map(|(i, j)| (i as usize, j as usize)) 
            {
                if maze[next_i][next_j] == '.' {
                    if next_i == 0 || next_j == 0 || next_i == n - 1 || next_j == m - 1 {
                        return distance + 1;
                    }
                    maze[next_i][next_j] = '+';
                    queue.push_back((next_i, next_j, distance + 1))
                }
            }
        }
        -1
    }
}

#[test]
fn test_1() {
    assert_eq!(
        1,
        Solution::nearest_exit(
            vec![vec!['+','+','.','+'], vec!['.','.','.','+'], vec!['+','+','+','.']],
            vec![1,2]
        )
    )
}

#[test]
fn test_2() {
    assert_eq!(
        2,
        Solution::nearest_exit(
            vec![vec!['+','+','+'], vec!['.','.','.'], vec!['+','+','+']],
            vec![1,0]
        )
    )
}

#[test]
fn test_3() {
    assert_eq!(
        -1,
        Solution::nearest_exit(
            vec![vec!['.','+']],
            vec![0,0]
        )
    )
}