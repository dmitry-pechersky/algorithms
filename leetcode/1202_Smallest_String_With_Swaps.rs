use std::collections::HashMap;

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

    fn join(&mut self, i: usize, j:usize) {
        let (root_i, root_j) = (self.find(i), self.find(j));
        if root_i != root_j {
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

}

impl Solution {
    pub fn smallest_string_with_swaps(s: String, pairs: Vec<Vec<i32>>) -> String {
        let mut s: Vec<char> = s.chars().collect();
        let n = s.len();
        let mut ds = DisjointSet::new(n);
        let mut groups: HashMap<usize, (Vec<usize>, Vec<char>)> = HashMap::new();
        for pair in pairs {
            let (i, j) = (pair[0] as usize, pair[1] as usize);
            ds.join(i, j);
        }
        for (idx, ch) in s.iter().enumerate() {
            let root = ds.find(idx);
            let (idxs, chs) = groups.entry(root).or_insert((vec![], vec![]));
            idxs.push(idx);
            chs.push(*ch);                
        }
        for (idxs, chs) in groups.values_mut() {
            chs.sort();
            for (idx, ch) in idxs.iter().zip(chs.iter()) {
                s[*idx] = *ch;
            }
        }
        s.into_iter().collect()
    }
}

#[test]
fn test_1() {
    assert_eq!(Solution::smallest_string_with_swaps("dcab".to_string(), vec![vec![0,3], vec![1,2]]), "bacd".to_string());
}

#[test]
fn test_2() {
    assert_eq!(Solution::smallest_string_with_swaps("dcab".to_string(), vec![vec![0,3], vec![1,2], vec![0,2]]), "abcd".to_string());
}

#[test]
fn test_3() {
    assert_eq!(Solution::smallest_string_with_swaps("cba".to_string(), vec![vec![0,1],vec![1,2]]), "abc".to_string());
}
