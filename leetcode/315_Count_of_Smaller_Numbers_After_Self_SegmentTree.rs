#[derive(Debug)]
pub struct SegmentTree { n: usize, tree: Vec<u32> }

impl SegmentTree { 
    pub fn new(size: usize) -> Self {
        let n = size.next_power_of_two();
        let tree = vec![0; (n - 1) + n];
        Self { n, tree }
    }
 
    pub fn increment(&mut self, i: usize) {
        let mut i = i + self.n - 1;
        loop {
            self.tree[i] += 1;
            if i == 0 {
                return;
            }
            i = (i - 1) / 2;
        }
    }

    pub fn prefix_sum(&self, i: usize) -> u32 {
        let (mut node, mut shift, mut n) = (0, 0, self.n);
        let mut sum = 0;
        loop {
            let (left, right) = (node * 2 + 1, node * 2 + 2);
            let half_n = n / 2;
            let (right_max_i, left_max_i) = (shift + n - 1, shift + half_n - 1);
            if i >= right_max_i {
                sum += self.tree[node];
                break;
            } else if i > left_max_i {
                sum += self.tree[left];
                node = right;
                shift += half_n;
            } else if i == left_max_i {
                sum += self.tree[left];
                break;
            } else {
                node = left;
            }
            n = half_n;
        }
        sum
    }
}

pub struct Solution {}

impl Solution {
    pub fn count_smaller(nums: Vec<i32>) -> Vec<i32> {
        const VALUE_SHIFT:i32 = 10000;
        const VALUE_SIZE: usize = VALUE_SHIFT as usize * 2 + 1;
        let mut tree = SegmentTree::new(VALUE_SIZE);
        let mut res = vec![0; nums.len()];
        for (i, num) in  nums.into_iter().enumerate().rev() {
            let shifted_value = (num + VALUE_SHIFT) as usize; 
            res[i] = if shifted_value == 0 { 0 } else { tree.prefix_sum(shifted_value - 1) as i32 };
            tree.increment(shifted_value);
        }
        res
    }
}

#[test]
fn test_segment_tree() {
    let mut tree = SegmentTree::new(3);
    tree.increment(0);
    tree.increment(1);
    tree.increment(1);
    assert_eq!(1, tree.prefix_sum(0));
    assert_eq!(3, tree.prefix_sum(1));
    assert_eq!(3, tree.prefix_sum(2));    
}


#[test]
fn test_1() {    
    assert_eq!(vec![2,1,0], Solution::count_smaller(vec![1,0,-1]));      
    assert_eq!(vec![2,1,1,0], Solution::count_smaller(vec![5,2,6,1]));
    assert_eq!(vec![0], Solution::count_smaller(vec![-1]));
    assert_eq!(vec![0, 0], Solution::count_smaller(vec![-1, -1]));
    assert_eq!(vec![0,0,0,0], Solution::count_smaller(vec![5,6,7,8]));
    assert_eq!(vec![2,4,4,2,2,0,0], Solution::count_smaller(vec![0,1,1,0,0,-1,-1]));

}