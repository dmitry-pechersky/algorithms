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
    pub fn earliest_acq(logs: Vec<Vec<i32>>, n: i32) -> i32 {
        let mut logs = logs;
        logs.sort_by(|a, b| a[0].cmp(&b[0]));
        let mut ds = DisjointSet::new(n as usize);
        let mut join_cnt = 0;
        for (timestamp, v, u) in logs.iter().map(|v| (v[0], v[1] as usize, v[2] as usize)) {
            if ds.find(v) != ds.find(u) {
                ds.join(v, u);
                join_cnt += 1;
                if join_cnt == n - 1 {
                    return timestamp;
                }
            } 
        }
        -1
    }
}

#[test]
fn test_1() {
    assert_eq!(
        20190301,
        Solution::earliest_acq(
            vec![vec![20190101,0,1],vec![20190104,3,4],vec![20190107,2,3],vec![20190211,1,5],vec![20190224,2,4],vec![20190301,0,3],vec![20190312,1,2],vec![20190322,4,5]], 
            6
        )   
    );
}

#[test]
fn test_2() {
    assert_eq!(
        3,
        Solution::earliest_acq(
            vec![vec![0,2,0],vec![1,0,1],vec![3,0,3],vec![4,1,2],vec![7,3,1]], 
            4
        )   
    );
}
