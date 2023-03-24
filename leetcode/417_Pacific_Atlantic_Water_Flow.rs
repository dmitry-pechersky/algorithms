struct Solution {}

impl Solution {
    pub fn pacific_atlantic(heights: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        fn dfs(heights: &Vec<Vec<i32>>, start: Vec<(usize, usize)>, can_flow: &mut Vec<Vec<bool>>) {
            let (n, m) =(heights.len(), heights[0].len());
            let mut stack = start;
            for &(i, j) in stack.iter() {
                can_flow[i][j] = true;
            }
            while let Some((i, j)) = stack.pop() {
                for (next_i, next_j) in [(1, 0), (0, 1), (-1, 0), (0, -1)].iter()
                    .map(|(di, dj)| (i as i32 + di, j as i32 + dj))
                    .filter(|&(i, j)| i >= 0 && i < n as i32 && j >= 0 && j < m as i32)
                    .map(|(i, j)| (i as usize, j as usize)) 
                    {
                        if heights[next_i][next_j] >= heights[i][j] && ! can_flow[next_i][next_j] {
                            can_flow[next_i][next_j] = true;
                            stack.push((next_i, next_j));
                        }
                    }
            }
        }

        let (n, m) =(heights.len(), heights[0].len());
        let mut pacific_flow = vec![vec![false; m]; n];
        let mut atlantic_flow = vec![vec![false; m]; n];
        let mut res = vec![];
        dfs(
            &heights, 
            (0..n).map(|i| (i, 0)).chain((0..m).map(|j| (0, j))).collect::<Vec<_>>(),
            &mut pacific_flow
        );
        dfs(
            &heights, 
            (0..n).map(|i| (i, m - 1)).chain((0..m).map(|j| (n - 1, j))).collect::<Vec<_>>(),
            &mut atlantic_flow
        );
        for i in 0..n {
            for j in 0..m {
                if atlantic_flow[i][j] && pacific_flow[i][j] {
                    res.push(vec![i as i32, j as i32]);
                }
            }
        }
        res
    }
}

#[test]
fn test_1() {
    let mut target = [[0,4],[1,3],[1,4],[2,2],[3,0],[3,1],[4,0]].iter().map(|a| a.to_vec()).collect::<Vec<_>>();
    let mut result = Solution::pacific_atlantic(
        [
            [1,2,2,3,5],
            [3,2,3,4,4],
            [2,4,5,3,1],
            [6,7,1,4,5],
            [5,1,1,2,4]
        ].iter().map(|a| a.to_vec()).collect()
    );
    target.sort();
    result.sort();

    assert_eq!(target, result);
}

#[test]
fn test_2() {
    assert_eq!(
        vec![vec![0,0]], 
        Solution::pacific_atlantic(vec![vec![1]])
    );
}
