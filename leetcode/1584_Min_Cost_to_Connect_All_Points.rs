struct Solution {}

struct DisjointSet {
    parents: Vec<usize>,
    ranks: Vec<usize>,
}

impl DisjointSet {
    fn new(n: usize) -> Self {
        Self { parents: (0..n).collect(), ranks: vec![0; n] }
    }

    fn find(&mut self, mut i: usize) -> usize {
        while i != self.parents[i] {
            let tmp = i;
            i = self.parents[i];
            self.parents[tmp] = self.parents[i];
        }
        i
    }

    fn join(&mut self, i: usize, j: usize) {
        let (root_i, root_j) = (self.find(i), self.find(j));
        if self.ranks[root_i] < self.ranks[root_j] {
            self.parents[root_i] = root_j;
        } else if self.ranks[root_i] > self.ranks[root_j] {
            self.parents[root_j] = root_i;
        } else {
            self.parents[root_i] = root_j;
            self.ranks[root_j] += 1;
        }
    }
}

impl Solution {
    pub fn min_cost_connect_points(points: Vec<Vec<i32>>) -> i32 {
        let n = points.len();
        let mut edges = Vec::new();
        let mut ds = DisjointSet::new(n);
        let mut total_cost = 0;
        let mut edges_cnt = 0;
        for i in 0..n {
            for j in i + 1..n {
                edges.push(((points[i][0] - points[j][0]).abs() + (points[i][1] - points[j][1]).abs(), i, j))
            }
        }
        edges.sort();
        for (cost, i, j) in edges {
            if ds.find(i) != ds.find(j) {
                ds.join(i, j);
                total_cost += cost;
                edges_cnt += 1;
                if edges_cnt == n - 1 {
                    break;
                }
            }
        }
        total_cost
    }
}

#[test]
fn test_1() {
    assert_eq!(Solution::min_cost_connect_points(vec![vec![0,0], vec![2,2], vec![3,10], vec![5,2], vec![7,0]]), 20);
}

#[test]
fn test_2() {
    assert_eq!(Solution::min_cost_connect_points(vec![vec![3,12], vec![-2,5], vec![-4,1]]), 18);
}

