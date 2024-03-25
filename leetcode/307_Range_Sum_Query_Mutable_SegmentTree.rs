pub struct NumArray {
    tree: Vec<i32>,
    shift: usize
}

impl NumArray {
    pub fn new(nums: Vec<i32>) -> Self {
        let n = nums.len();
        let mut tree = nums;
        let shift = n.next_power_of_two() - 1;
        tree.resize(n.next_power_of_two() * 2 - 1, 0);
        tree.rotate_right(shift);
        for i in (1..tree.len()).rev() {
            let parent = (i - 1) / 2;
            tree[parent] += tree[i];
        }
        Self { tree, shift }
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
        let mut delta = 0;
        {
            let mut idx = self.shift + left as usize;
            while idx != 0 {
                let parent = (idx - 1) / 2;
                if idx % 2 == 0 {
                    delta += self.tree[parent] - self.tree[idx];
                }
                idx = parent;
            }    
        }
        {
            let mut idx = self.shift + right as usize;
            while idx != 0 {
                let parent = (idx - 1) / 2;
                if idx % 2 == 1 {
                    delta += self.tree[parent] - self.tree[idx];
                }
                idx = parent;
            }    
        }
        self.tree[0] - delta
    }
}


#[test]
fn test_num_array() {
    let mut array = NumArray::new(vec![1, 3, 5]);
    assert_eq!(9, array.sum_range(0, 2));
    array.update(1, 2);
    assert_eq!(8, array.sum_range(0, 2));
}