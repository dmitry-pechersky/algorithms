struct DisjointSet {
    parents: Vec<usize>,
    ranks: Vec<i32>,
}

impl  DisjointSet {
    fn new(n: usize) -> Self {
        DisjointSet { parents: (0..n).collect(), ranks: vec![0; n] }
    }

    fn find(&mut self, v: usize) -> usize {
        let mut cur = v;
        while self.parents[cur] != cur {
            let prev = cur;
            cur = self.parents[cur];
            self.parents[prev] = self.parents[cur];
        }
        cur
    }

    fn join(&mut self, v: usize, u: usize) {
        let (root_v, root_u) = (self.find(v), self.find(u));
        if root_v != root_u {
            if self.ranks[root_v] < self.ranks[root_u] {
                self.parents[root_v] = root_u;
            } else {
                self.parents[root_u] = root_v;
                if self.ranks[root_v] == self.ranks[root_u] {
                    self.ranks[root_v] += 1;
                }
            }
        }
    }
}

struct Solution {}

impl Solution {
    pub fn num_islands2(n: i32, m: i32, positions: Vec<Vec<i32>>) -> Vec<i32> {
        fn neibors(i: usize, j: usize, n: usize, m: usize) -> impl Iterator<Item=(usize, usize)> {
            [(1,0), (0, 1), (-1, 0), (0, -1)].iter()
                .map(move |(di, dj)| (i as i32 + di, j as i32 + dj))
                .filter(move |&(i, j)| i >= 0 && i < n as i32 && j >= 0 && j < m as i32)
                .map(|(i, j)| (i as usize, j as usize))
        }

        let (n, m) = (n as usize, m as usize);
        let mut ds = DisjointSet::new((n * m) as usize);
        let mut res = vec![];
        let mut grid = vec![vec![false; m]; n];
        let mut cnt = 0;
        for (i, j) in positions.iter().map(|v| (v[0] as usize, v[1] as usize)) {
            if ! grid[i][j] {
                grid[i][j] = true;
                cnt += 1;
                let idx = i * m + j;
                for (neibor_i, neibor_j) in neibors(i, j, n, m) {
                    if grid[neibor_i][neibor_j] {
                        let neibor_idx = neibor_i * m + neibor_j;
                        if ds.find(idx) != ds.find(neibor_idx) {
                            ds.join(idx, neibor_idx);
                            cnt -= 1;
                        }
                    }
                }
            }
            res.push(cnt);
        }
        res
    }
}

#[test]
fn test_1() {
    assert_eq!(
        vec![1,1,2,3],
        Solution::num_islands2(3, 3, vec![vec![0,0],vec![0,1],vec![1,2],vec![2,1]])
    );
}


#[test]
fn test_2() {
    assert_eq!(
        vec![1],
        Solution::num_islands2(1, 1, vec![vec![0,0]])
    );
}

#[test]
fn test_3() {
    assert_eq!(
        vec![1, 1],
        Solution::num_islands2(1, 2, vec![vec![0,1], vec![0,0]])
    );
}
