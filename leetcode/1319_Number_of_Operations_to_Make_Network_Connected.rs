struct DisjointSet {
    parents: Vec<usize>,
    ranks: Vec<u32>
}

impl DisjointSet {
    fn new(n: usize) -> Self {
        Self { parents: (0..n).into_iter().collect(), ranks: vec![1; n] }
    }

    fn find(&mut self, value: usize) -> usize {
        let mut value = value;
        while value != self.parents[value] {
            let parent = self.parents[value];
            self.parents[value] = self.parents[parent];
            value = parent;
        }
        value
    }

    fn union(&mut self, a: usize, b: usize) {
        let a_root = self.find(a);
        let b_root = self.find(b);
        if a_root != b_root {
            if self.ranks[a_root] > self.ranks[b_root] {
                self.parents[b_root] = a_root;
            } else if self.ranks[a_root] < self.ranks[b_root] {
                self.parents[a_root] = b_root;
            } else {
                self.parents[a_root] = b_root;
                self.ranks[b_root] += 1;
            }
        }
    }
}

struct Solution { }

impl Solution {
    pub fn make_connected(n: i32, connections: Vec<Vec<i32>>) -> i32 {
        if connections.len() < n as usize - 1 {
            return -1;
        }
        let mut disjoint_set = DisjointSet::new(n as usize);
        let mut discarded_cnt = 0;
        for (v, u) in connections.iter().map(|v| (v[0] as usize, v[1] as usize)) {
            if disjoint_set.find(v) != disjoint_set.find(u) {
                disjoint_set.union(v, u);
            } else {
                discarded_cnt += 1;
            }
        }
        (n as i32 - 1) - (connections.len() as i32 - discarded_cnt)
    }
}

#[test]
fn test_1() {
    assert_eq!(1, Solution::make_connected(4, vec![vec![0,1],vec![0,2],vec![1,2]]));
    assert_eq!(2, Solution::make_connected(6, vec![vec![0,1],vec![0,2],vec![0,3],vec![1,2],vec![1,3]]));
    assert_eq!(-1, Solution::make_connected(6, vec![vec![0,1],vec![0,2],vec![0,3],vec![1,2]]));
}
