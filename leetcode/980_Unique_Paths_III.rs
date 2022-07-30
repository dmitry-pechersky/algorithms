struct Solution {}

impl Solution {
    pub fn unique_paths_iii(mut grid: Vec<Vec<i32>>) -> i32 {
        fn dfs(x: usize, y: usize, grid: &Vec<Vec<i32>>, n: usize, m: usize, empty_cnt: u32, in_stack: &mut Vec<Vec<bool>>, in_stack_cnt: &mut u32, path_cnt: &mut i32)  {
            in_stack[x][y] = true;
            *in_stack_cnt += 1;
            for (x1, y1) in [(0, 1), (1, 0), (0, -1), (-1, 0)].map(|(dx, dy)| (x as isize + dx, y as isize + dy)) {
                if  0 <= x1 && x1 < n as isize && 0 <= y1 && y1 < m as isize  {
                    let (x1, y1) = (x1 as usize, y1 as usize);
                    if grid[x1][y1] == 0 && ! in_stack[x1][y1] {
                        dfs(x1, y1, grid, n, m, empty_cnt, in_stack, in_stack_cnt, path_cnt);               
                    } else if grid[x1][y1] == 2 && *in_stack_cnt == empty_cnt + 1 {
                        *path_cnt += 1;
                    }

                }
            }            
            in_stack[x][y] = false;
            *in_stack_cnt -= 1;
        }

        let (n, m) = (grid.len(), grid[0].len());
        let mut path_cnt = 0;
        let mut empty_cnt = 0;
        let (mut start_x, mut start_y) = (0, 0);
        for x in 0..n {
            for y in 0..m {
                if grid[x][y] == 0 {
                    empty_cnt += 1;
                } else if grid[x][y] == 1 {
                    start_x = x;
                    start_y = y;
                }
            }
        }
        let mut in_stack = vec![vec![false; m]; n];
        let mut in_stack_cnt = 0;
        dfs(start_x, start_y, &mut grid, n, m, empty_cnt, &mut in_stack, &mut in_stack_cnt, &mut path_cnt);
        path_cnt
    }
}

#[test]
fn test_1() {
    assert_eq!(2, Solution::unique_paths_iii(vec![vec![1,0,0,0],vec![0,0,0,0],vec![0,0,2,-1]]));
}

#[test]
fn test_2() {
    assert_eq!(4, Solution::unique_paths_iii(vec![vec![1,0,0,0],vec![0,0,0,0],vec![0,0,0,2]]));
}

#[test]
fn test_3() {
    assert_eq!(0, Solution::unique_paths_iii(vec![vec![0,1],vec![2,0]]));
}