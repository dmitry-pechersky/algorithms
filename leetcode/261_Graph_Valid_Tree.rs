struct Solution {}

struct DisjointSet {
    parents: Vec<usize>,
    ranks: Vec<i32>,
}

impl DisjointSet {
    fn new(n: usize) -> Self {
        Self { parents: (0..n).collect(), ranks: vec![1; n] }
    }

    fn find(&mut self, v: usize) -> usize {
        let mut cur = v;
        while self.parents[cur] != cur {
            cur = self.parents[cur];
        }
        let root = cur;

        let mut cur= v;
        while self.parents[cur] != root {
            let parent = self.parents[cur];
            self.parents[cur] = root;
            cur = parent;
        }
        root 
    }

    fn join(&mut self, v: usize, u: usize) {
        let root_v = self.find(v);
        let root_u = self.find(u);
        if root_u != root_v {
            if self.ranks[root_v] > self.ranks[root_u] {
                self.parents[root_u] = root_v;
            } else {
                self.parents[root_v] = root_u;
                if self.ranks[root_v] == self.ranks[root_u] {
                    self.ranks[root_u] += 1;
                }
            } 
        }
    }
}

impl Solution {
    pub fn valid_tree(n: i32, edges: Vec<Vec<i32>>) -> bool {
        let mut ds = DisjointSet::new(n as usize);
        for (u, v) in edges.into_iter().map(|v| (v[0] as usize, v[1] as usize)) {
            let (root_v, root_u) = (ds.find(v), ds.find(u));
            if root_v == root_u {
                return false;
            }
            ds.join(root_v, root_u);
        }
        let root = ds.find(0);
        (0..n as usize).all(|v| ds.find(v) == root)
    }
}

#[test]
fn test_1() {
    assert!(Solution::valid_tree(5, vec![vec![0,1],vec![0,2],vec![0,3],vec![1,4]]))
}

#[test]
fn test_2() {
    assert!(!Solution::valid_tree(5, vec![vec![0,1],vec![1,2],vec![2,3],vec![1,3],vec![1,4]]));
}
