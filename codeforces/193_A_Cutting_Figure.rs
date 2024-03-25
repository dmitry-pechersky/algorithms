use std::io::{stdin, BufRead};

#[derive(Clone, PartialEq, Debug)]
enum DFSColor { White, Grey, Black }

fn read_input() -> Vec<Vec<bool>> {
    let mut input = stdin().lock().lines();
    let n = input.next().unwrap().unwrap().trim().split_ascii_whitespace().next().unwrap().parse().unwrap();
    (0..n).map(|_| input.next().unwrap().unwrap().trim().bytes().map(|ch| ch == b'#').collect()).collect()
}

fn has_articulation_point(grid: &[Vec<bool>]) -> bool {
    fn dfs(
        x: usize, y: usize, n: usize, m: usize, grid: &[Vec<bool>], colors: &mut [Vec<DFSColor>],
        t: usize, t_in: &mut [Vec<usize>], ret: &mut [Vec<usize>]
    ) -> bool
    {      
        colors[x][y] = DFSColor::Grey;
        t_in[x][y] = t;
        ret[x][y] = t;
        let mut branch_cnt = 0;

        for (next_x, next_y) in [(0, 1), (1, 0), (0, -1), (-1, 0)].iter()
            .map(|(dx, dy)| (x as isize + dx, y as isize + dy))
            .filter(|&(x, y)| 0 <= x && x < n as isize && 0 <= y && y < m as isize)
            .map(|(x, y)| (x as usize, y as usize))
        {
            if grid[next_x][next_y] {
                match  colors[next_x][next_y] {
                    DFSColor::White => {
                        if dfs(next_x, next_y, n, m, grid, colors, t + 1, t_in, ret) {
                            return true;
                        }
                        ret[x][y] = ret[x][y].min(ret[next_x][next_y]);
                        if t_in[x][y] <= ret[next_x][next_y] {
                            branch_cnt += 1;
                        }
                    },
                    DFSColor::Grey => {
                        ret[x][y] = ret[x][y].min(t_in[next_x][next_y]);
                    },
                    DFSColor::Black => { }
                }
            }
        }
        colors[x][y] = DFSColor::Black;
        (t == 0 && branch_cnt > 1) || (t > 0 && branch_cnt > 0 )
    }

    let (n, m) = (grid.len(), grid[0].len());
    let mut colors = vec![vec![DFSColor::White; m]; n];
    let mut t_in = vec![vec![0; m]; n];
    let mut ret = vec![vec![0; m]; n];

    for x in 0..n {
        for y in 0..m {
            if grid[x][y] && colors[x][y] == DFSColor::White && dfs(x, y, n, m, grid, &mut colors, 0, &mut t_in, &mut ret) {
                return true;
            }
        }
    }
    false
}

fn main() {
    let grid = read_input();
    let node_cnt = grid.iter().flat_map(|row| row.iter()).filter(|v| **v).count() ;
    let res = if node_cnt < 3 {
        -1
    } else if has_articulation_point(&grid) { 
        1 
    } else { 
        2 
    };
    println!("{}", res);
}