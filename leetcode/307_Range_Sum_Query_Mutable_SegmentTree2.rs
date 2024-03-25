pub struct NumArray {
    tree: Vec<i32>,
    left: Vec<usize>,
    right: Vec<usize>,
    shift: usize
}

impl NumArray {
    pub fn new(nums: Vec<i32>) -> Self {
        let n = nums.len();
        let (shift, tree_len) = (n.next_power_of_two() - 1, n.next_power_of_two() * 2 - 1);
        let (mut tree, mut left, mut right) = (vec![0; tree_len], vec![0; tree_len], vec![0; tree_len]);

        tree[shift..shift + n].copy_from_slice(&nums);

        for i in 0.. shift + 1 {
            left[shift + i] = i;
            right[shift + i] = i;
        }

        for idx in (1..tree.len()).rev() {
            let parent = (idx - 1) / 2;
            tree[parent] += tree[idx];
            if idx % 2 == 0 {
                right[parent] = right[idx];
            } else {
                left[parent] = left[idx];
            }
        }

        Self { tree, left, right, shift }
    }
    
    pub fn update(&mut self, index: i32, val: i32) {
        let mut idx = self.shift + index as usize;
        let change = val - self.tree[idx];
        self.tree[idx] = val;
        while idx != 0 {
            let parent = (idx - 1) / 2;
            self.tree[parent] += change;
            idx = parent;
        }
    }
    
    pub fn sum_range(&self, left: i32, right: i32) -> i32 {
        self.sum_range_rec(0, left as usize, right as usize)
    }

    fn sum_range_rec(&self, idx: usize, left: usize, right: usize) -> i32 {
        if right < self.left[idx] || left > self.right[idx] {
            0
        } else if left <= self.left[idx] && self.right[idx] <= right {
            self.tree[idx]
        } else {
            let (left_child, right_child) = (idx * 2 + 1, idx * 2 + 2);
            self.sum_range_rec(left_child, left, right) + self.sum_range_rec(right_child, left, right)
        }
    }
}


#[test]
fn test_num_array() {
    let mut array = NumArray::new(vec![1, 3, 5]);
    assert_eq!(9, array.sum_range(0, 2));
    array.update(1, 2);
    assert_eq!(8, array.sum_range(0, 2));
}