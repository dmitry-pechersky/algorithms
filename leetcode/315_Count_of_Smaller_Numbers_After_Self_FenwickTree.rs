pub struct FenwickTree {
    tree: Vec<u32>
}

impl  FenwickTree {
    pub fn new(size: usize) -> Self {
        Self { tree: vec![0; size] }
    }

    pub fn increment(&mut self, i: usize) {
        let n = self.tree.len();
        let mut i = i;
        while i < n {
            self.tree[i] += 1;
            i = i | (i + 1);
        }
    }

    pub fn prefix_sum(&self, i: usize) -> u32 {
        let mut sum = 0;
        let mut i = i;
        loop {
            sum += self.tree[i];
            i = i & (i + 1);
            if i == 0 {
                break;
            }
            i -= 1;
        }
        sum
    }
}

pub struct Solution {}

impl Solution {
    pub fn count_smaller(nums: Vec<i32>) -> Vec<i32> {
        const VALUE_SHIFT:i32 = 10000;
        const VALUE_SIZE: usize = 20001;
        let mut tree = FenwickTree::new(VALUE_SIZE);
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
fn test_1() {    
    assert_eq!(vec![2,1,1,0], Solution::count_smaller(vec![5,2,6,1]));
    assert_eq!(vec![0], Solution::count_smaller(vec![-1]));
    assert_eq!(vec![0, 0], Solution::count_smaller(vec![-1, -1]));
    assert_eq!(vec![0,0,0,0], Solution::count_smaller(vec![5,6,7,8]));
    assert_eq!(vec![2,1,0], Solution::count_smaller(vec![1,0,-1]));
    assert_eq!(vec![2,4,4,2,2,0,0], Solution::count_smaller(vec![0,1,1,0,0,-1,-1]));
}