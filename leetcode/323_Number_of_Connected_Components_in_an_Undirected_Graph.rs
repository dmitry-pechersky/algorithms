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
    pub fn count_components(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let mut ds = DisjointSet::new(n as usize);
        let mut cnt = 0;
        for (v, u) in edges.iter().map(|v| (v[0] as usize, v[1] as usize)) {
            if ds.find(v) != ds.find(u) {
                ds.join(v, u);
                cnt += 1;
            }
        }
        n - cnt
    }
}

#[test]
fn test_1() {
    assert_eq!(
        2,
        Solution::count_components(5, vec![vec![0,1],vec![1,2],vec![3,4]])
    );
}

#[test]
fn test_2() {
    assert_eq!(
        1,
        Solution::count_components(5, vec![vec![0,1],vec![1,2],vec![2,3],vec![3,4]])
    );
}