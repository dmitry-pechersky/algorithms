mod dp {
    pub struct Solution {}

    impl Solution {
        pub fn find_number_of_lis(nums: Vec<i32>) -> i32 {
            let n = nums.len();
            let mut dp_length = vec![0; n];
            let mut dp_cnt  = vec![0; n];
            for i in 0..n {
                let (mut length,  mut cnt) = (0, 1);
                for j in 0..i {
                    if nums[j] < nums[i] {
                        if dp_length[j] > length {
                            length = dp_length[j];
                            cnt = dp_cnt[j];
                        } else if dp_length[j] == length {
                            cnt += dp_cnt[j];
                        }    
                    }
                }
                dp_length[i] = length + 1;
                dp_cnt[i] = cnt;
            }
            let (mut length, mut cnt) = (0, 0);
            for i in 0..n {
                if dp_length[i] == length {
                    cnt += dp_cnt[i];
                } else if dp_length[i] > length {
                    length = dp_length[i];
                    cnt = dp_cnt[i];
                }
            }
            cnt
        }
    }    
    
    #[test]
    fn test_find_number_of_lis() {
        assert_eq!(2, Solution::find_number_of_lis(vec![1,3,5,4,7]));
        assert_eq!(5, Solution::find_number_of_lis(vec![2,2,2,2,2]));
    }
}

mod segment_tree {
    use std::cmp::Reverse;

    pub struct SegmentTree { n: usize, max: Vec<u16>, cnt: Vec<u32> }


    impl SegmentTree {
        pub fn new(size: u16) -> Self {
            let n = size.next_power_of_two() as usize;
            Self { n, max: vec![0; n * 2 - 1], cnt: vec![0; n * 2 - 1] }
        }

        pub fn update(&mut self, i: u16, value: u16, cnt: u32) {
            let mut i = self.n - 1 + i as usize;
            self.max[i] = value;
            self.cnt[i] = cnt;
            while i > 0 {
                i = (i - 1) / 2;
                let (left, right) = (i * 2 + 1, i * 2 + 2);
                if self.max[left] > self.max[right] {
                    self.max[i] = self.max[left];
                    self.cnt[i] = self.cnt[left]; 
                } else if self.max[left] < self.max[right] {
                    self.max[i] = self.max[right];
                    self.cnt[i] = self.cnt[right]; 
                } else {
                    self.max[i] = self.max[right];
                    self.cnt[i] = self.cnt[left] + self.cnt[right];                 
                }
            }
        }

        pub fn prefix_max_cnt(&self, i: u16) -> (u16, u32) {
            self.prefix_max_cnt_rec(0, i as usize, 0, self.n)
        }

        fn prefix_max_cnt_rec(&self, node: usize, i: usize, shift: usize, n: usize) -> (u16, u32) {
            let (left, right) = (node * 2 + 1, node * 2 + 2);
            let (left_lower_i, right_upper_i) = (shift, shift + n - 1);
            if i >= right_upper_i {
                (self.max[node], self.cnt[node])
            } else if i >= left_lower_i {
                let (max1, cnt1) = self.prefix_max_cnt_rec(left, i, shift, n / 2);
                let (max2, cnt2) = self.prefix_max_cnt_rec(right, i, shift + n / 2, n / 2);
                if max1 > max2 {
                    (max1, cnt1)
                } else if max1 < max2 {
                    (max2, cnt2)
                } else {
                    (max1, cnt1 + cnt2)
                }
            } else {
                (0, 0)
            }
        }
    }

    pub struct Solution {}

    impl Solution {
        pub fn find_number_of_lis(nums: Vec<i32>) -> i32 {
            let n = nums.len();
            let mut nums = nums.into_iter().enumerate().map(|(i, a)| (a, i as u16)).collect::<Vec<_>>();
            nums.sort_unstable_by_key(|&(a, i)| (a, Reverse(i)));
            let mut tree = SegmentTree::new(nums.len() as u16);
            for (_, i) in nums {
                let (length, cnt) = tree.prefix_max_cnt(i);
                tree.update(i, length + 1, cnt.max(1));
            }
            tree.prefix_max_cnt(n as u16 - 1).1 as i32
        }
    }

    #[test]
    fn test_segment_tree() {
        let mut tree = SegmentTree::new(6);
        tree.update(0, 5, 1);
        tree.update(1, 3, 1);
        tree.update(2, 5, 1);
        tree.update(3, 6, 1);
        tree.update(4, 6, 1);
        assert_eq!((6, 1), tree.prefix_max_cnt(3));
        assert_eq!((5, 2), tree.prefix_max_cnt(2));
        assert_eq!((5, 1), tree.prefix_max_cnt(1));
        assert_eq!((5, 1), tree.prefix_max_cnt(0));
    }

    #[test]
    fn test_find_number_of_lis() {
        assert_eq!(2, Solution::find_number_of_lis(vec![1,2,2]));        
        assert_eq!(2, Solution::find_number_of_lis(vec![1,3,5,4,7]));
        assert_eq!(5, Solution::find_number_of_lis(vec![2,2,2,2,2]));
        
    }
}

mod fenwick_tree {
    use std::cmp::Reverse;

    #[derive(Debug)]
    pub struct FenwickTree { max_value: Vec<u16>, cnt: Vec<u32> }

    impl FenwickTree {
        pub fn zero(n: usize) -> Self {
            Self { max_value: vec![0; n], cnt: vec![0; n] }
        }

        pub fn add(&mut self, i: usize, value: u16, cnt: u32) {
            let mut i = i;
            while i < self.max_value.len() {
                if value > self.max_value[i] {
                    self.max_value[i] = value;
                    self.cnt[i] = cnt;
                } else if value == self.max_value[i] {
                    self.cnt[i] += cnt;
                }
                i = i | (i + 1);
            }
        }

        pub fn prefix_max_cnt(&self, i: usize) -> (u16, u32) {
            let mut i = i;
            let (mut max_value, mut max_value_cnt) = (0, 0);
            loop {
                if self.max_value[i] > max_value {
                    max_value = self.max_value[i];
                    max_value_cnt = self.cnt[i];
                } else if self.max_value[i] == max_value {
                    max_value_cnt += self.cnt[i];
                }
                i = i & (i + 1);
                if i == 0 {
                    break;
                }
                i -= 1;
            }
            (max_value, max_value_cnt)
        }
    }

    pub struct Solution {}

    impl Solution {
        pub fn find_number_of_lis(nums: Vec<i32>) -> i32 {
            let n = nums.len();
            let mut nums = nums.into_iter().enumerate().map(|(i, a)| (a, i as u16)).collect::<Vec<_>>();
            nums.sort_unstable_by_key(|&(a, i)| (a, Reverse(i)));
            let mut tree = FenwickTree::zero(nums.len());
            for (_, i) in nums {
                let (length, cnt) = tree.prefix_max_cnt(i as usize);
                tree.add(i as usize, length + 1, cnt.max(1));
            }
            tree.prefix_max_cnt(n - 1).1 as i32
        }
    }        

    #[test]
    fn test_find_number_of_lis() {
        assert_eq!(6, Solution::find_number_of_lis(vec![1,1,2,2,2,3]));        
        assert_eq!(2, Solution::find_number_of_lis(vec![1,2,2]));        
        assert_eq!(2, Solution::find_number_of_lis(vec![1,3,5,4,7]));
        assert_eq!(5, Solution::find_number_of_lis(vec![2,2,2,2,2]));
        assert_eq!(27, Solution::find_number_of_lis(vec![1,1,1,2,2,2,3,3,3]));        
    }
}