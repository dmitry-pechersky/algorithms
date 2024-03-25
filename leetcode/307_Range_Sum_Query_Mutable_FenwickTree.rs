pub struct NumArray {
    tree: Vec<i32>
}

impl NumArray {
    pub fn new(nums: Vec<i32>) -> Self {
        let n = nums.len();
        let mut nums = nums;
        for i in 1..n {
            nums[i] += nums[i - 1];
        }
        let tree = (0..n).map(
            |i| {
                let fi = i & (i + 1);
                if fi == 0 { nums[i] } else { nums[i] - nums[fi - 1] }    
            }
        ).collect();
        Self { tree }        
    }
    
    pub fn update(&mut self, index: i32, val: i32) {
        let mut idx = index as usize;
        let delta = val - self.sum_range_p(idx, idx);
        while idx < self.tree.len() {
            self.tree[idx] += delta;
            idx = idx | (idx + 1);
        }
    }
    
    pub fn sum_range(&self, left: i32, right: i32) -> i32 {
        self.sum_range_p(left as usize, right as usize)
    }

    fn sum_range_p(&self, left: usize, right: usize) -> i32 {
        if left == 0 {
            self.sum_prefix(right)
        } else {
            self.sum_prefix(right) - self.sum_prefix(left - 1)
        }        
    }

    fn sum_prefix(&self, mut idx: usize) -> i32 {
        let mut sum = 0;
        loop {
            sum += self.tree[idx];
            let f = idx & (idx + 1);
            if f == 0 {
                break
            }        
            idx = f - 1;                
        }
        sum
    }
}


#[test]
fn test_num_array() {
    let mut array = NumArray::new(vec![1, 3, 5]);
    assert_eq!(9, array.sum_range(0, 2));
    assert_eq!(3, array.sum_range(1, 1));
    array.update(1, 2);
    println!("{:?}", array.tree);
    assert_eq!(8, array.sum_range(0, 2));
}