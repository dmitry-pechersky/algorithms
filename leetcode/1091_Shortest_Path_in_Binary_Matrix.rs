use std::collections::VecDeque;
struct Solution {}

impl Solution {
    pub fn shortest_path_binary_matrix(mut grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len() as i32;
        if grid[0][0] == 0 {
            let dxdy: Vec<(i32, i32)> = (-1..=1).into_iter()
                .flat_map(|dx| (-1..=1).into_iter().map(move |dy| (dx, dy)))
                .filter(|(dx, dy)| *dx != 0 || *dy != 0 )
                .collect();
            grid[0][0] = 1;
            let mut queue = VecDeque::from([(1, 0, 0)]);
            while let Some((cost, x, y)) = queue.pop_front() {
                if x == n - 1 && y == n - 1 {
                    return cost
                }
                for (x, y) in  dxdy.iter()
                    .map(|(dx, dy)| (x + dx, y + dy))
                    .filter(|(x, y)|  0 <= *x && *x  < n && 0 <= *y && *y < n) 
                {
                    if grid[x as usize][y as usize] == 0 {
                        queue.push_back((cost + 1, x, y));
                        grid[x as usize][y as usize] = 1;    
                    }
                }
            }
        }
        -1
    }
}

#[test]
fn test_1() {
    assert_eq!(2, Solution::shortest_path_binary_matrix(vec![vec![0,1], vec![1,0]]));
}

#[test]
fn test_2() {
    assert_eq!(4, Solution::shortest_path_binary_matrix(vec![vec![0,0,0], vec![1,1,0], vec![1,1,0]]));
}

#[test]
fn test_3() {
    assert_eq!(-1, Solution::shortest_path_binary_matrix(vec![vec![1,0,0], vec![1,1,0], vec![1,1,0]]));
}
