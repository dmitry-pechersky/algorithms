struct Solution {}

impl Solution {
    pub fn count_sub_islands(grid1: Vec<Vec<i32>>, grid2: Vec<Vec<i32>>) -> i32 {
        let (n, m) = (grid1.len(), grid1[0].len());
        let mut grid2 = grid2;
        let mut stack = vec![];
        let mut subisland_cnt = 0;
        for i in 0..n {
            for j in 0..m {
                if grid2[i][j] == 1 {
                    let mut is_subisland = true;
                    grid2[i][j] = 0;
                    stack.push((i, j));
                    while let Some((i, j)) = stack.pop() {
                        if grid1[i][j] == 0 {
                            is_subisland = false;
                        }
                        for (next_i, next_j) in [(1, 0), (0, 1), (-1, 0), (0, -1)].iter()
                            .map(|(di, dj)| (i as i32 + di, j as i32 + dj))
                            .filter(|&(i, j)| i >= 0 && i < n as i32 && j >= 0 && j < m as i32)
                            .map(|(i, j)| (i as usize, j as usize)) 
                        {
                            if grid2[next_i][next_j] == 1 {
                                grid2[next_i][next_j] = 0;
                                stack.push((next_i, next_j));
                            }
                        }
                    }
                    if is_subisland {
                        subisland_cnt += 1;
                    }
                }
            }
        }
        subisland_cnt
    }
}

#[test]
fn test_1() {
    assert_eq!(
        3,
        Solution::count_sub_islands(
            [
                [1,1,1,0,0],
                [0,1,1,1,1],
                [0,0,0,0,0],
                [1,0,0,0,0],
                [1,1,0,1,1]
            ].iter().map(|a| a.to_vec()).collect(), 
            [
                [1,1,1,0,0],
                [0,0,1,1,1],
                [0,1,0,0,0],
                [1,0,1,1,0],
                [0,1,0,1,0]
            ].iter().map(|a| a.to_vec()).collect()
        )
    );
}

#[test]
fn test_2() {
    assert_eq!(
        2,
        Solution::count_sub_islands(
            [
                [1,0,1,0,1],
                [1,1,1,1,1],
                [0,0,0,0,0],
                [1,1,1,1,1],
                [1,0,1,0,1]
            ].iter().map(|a| a.to_vec()).collect(), 
            [
                [0,0,0,0,0],
                [1,1,1,1,1],
                [0,1,0,1,0],
                [0,1,0,1,0],
                [1,0,0,0,1]
            ].iter().map(|a| a.to_vec()).collect()
        )
    );
}
