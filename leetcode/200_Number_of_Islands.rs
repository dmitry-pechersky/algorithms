struct Solution {}

impl Solution {
    pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
        let (n, m) = (grid.len(), grid[0].len());
        let mut grid = grid;
        let mut stack = vec![];
        let mut cnt = 0;
        for i in 0..n {
            for j in 0..m {
                if grid[i][j] == '1' {
                    cnt += 1;
                    stack.push((i, j));
                    grid[i][j] = '0';
                    while let Some((i, j)) = stack.pop() {
                        for (next_i, next_j) in [(1, 0), (0, 1), (-1,0), (0, -1)].iter().map(|(di, dj)| (i as i32 + di, j as i32 + dj)) {
                            if next_i >= 0 && next_j >= 0 && next_i >= 0 {
                                let (next_i, next_j) = (next_i as usize, next_j as usize);
                                if next_i < n && next_j < m && grid[next_i][next_j] == '1' {
                                    grid[next_i][next_j] = '0';
                                    stack.push((next_i, next_j));
                                }
                            }
                        }
                    }
                }
            }
        }      
        cnt
    }
}

#[test]
fn test_1() {
    assert_eq!(
        1,
        Solution::num_islands(
            vec![
                vec!['1','1','1','1','0'],
                vec!['1','1','0','1','0'],
                vec!['1','1','0','0','0'],
                vec!['0','0','0','0','0']
            ]
        )
    );
}

#[test]
fn test_2() {
    assert_eq!(
        3,
        Solution::num_islands(
            vec![
                vec!['1','1','0','0','0'],
                vec!['1','1','0','0','0'],
                vec!['0','0','1','0','0'],
                vec!['0','0','0','1','1']
            ]
        )
    );
}
