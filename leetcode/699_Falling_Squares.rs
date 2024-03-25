mod simple {
    use std::collections::HashMap;

    pub struct Solution;

    impl Solution {
        fn sorted_position_dic(positions: &Vec<Vec<i32>>) -> HashMap<i32, usize> {
            let mut coordinates = positions.iter().map(|position| [position[0], position[0] + position[1] - 1]).flatten().collect::<Vec<_>>();
            coordinates.sort_unstable();
            let mut dic = HashMap::with_capacity(positions.len() * 2);
            for coordinate in coordinates {
                if ! dic.contains_key(&coordinate) {
                    dic.insert(coordinate, dic.len());
                }
            }
            dic    
        }

        pub fn falling_squares(positions: Vec<Vec<i32>>) -> Vec<i32> {            
            let n = positions.len();
            let dic  = Self::sorted_position_dic(&positions);
            let mut heights = vec![0; dic.len()];
            let mut max_height = 0;
            let mut max_heights = Vec::with_capacity(n);
            for position in positions {
                let (&x1, &x2) = (dic.get(&position[0]).unwrap(), dic.get(&(position[0] + position[1] - 1)).unwrap());
                let height = position[1];
                let max_interval_height = *heights[(x1..=x2)].iter().max().unwrap();
                let next_height = max_interval_height + height;
                for x in x1..=x2 {
                    heights[x] = next_height;
                }
                max_height = max_height.max(next_height);
                max_heights.push(max_height);
            }
            max_heights
        }
    }

    #[test]
    fn test_falling_squares() {
        assert_eq!(vec![2,5,5], Solution::falling_squares(vec![vec![1,2],vec![2,3],vec![6,1]]));
        assert_eq!(vec![100,100], Solution::falling_squares(vec![vec![100,100],vec![200,100]]));
    }
}

mod segment_tree {
    use std::collections::HashMap;

    pub struct MaxSegmentTree { n: usize, values: Vec<u32>, inconsistency: Vec<Option<u32>>}

    impl MaxSegmentTree {
        pub fn new(size: usize) -> Self {
            let n = size.next_power_of_two();
            Self { n, values: vec![0; 2 * n - 1], inconsistency: vec![None; n - 1] }
        }

        pub fn update_range(&mut self, range_left: usize, range_right: usize, value: u32) {
            self.update_range_rec(0, 0, self.n - 1, range_left, range_right, value);
        }

        pub fn query_range(&mut self, range_left: usize, range_right: usize) -> u32 {
            self.query_range_rec(0, 0, self.n - 1, range_left, range_right)
        }

        fn query_range_rec(&mut self, node: usize, left: usize, right: usize, range_left: usize, range_right: usize) -> u32 {
            if range_right < left || range_left > right {
                0
            } else if range_left <= left && right <= range_right {
                self.values[node]
            } else {
                self.push_inconsistency(node);
                let (left_child, right_child) = (2 * node + 1, 2 * node + 2);
                let left_query_res = self.query_range_rec(left_child, left, (right + left) / 2, range_left, range_right);
                let right_query_res = self.query_range_rec(right_child, (right + left) / 2 + 1, right, range_left, range_right);
                left_query_res.max(right_query_res)
            }
        }

        fn update_range_rec(&mut self, node: usize, left: usize, right: usize, range_left: usize, range_right: usize, value: u32) {
            if range_right < left || range_left > right {
                return;
            }            
            if left == right {
                self.values[node] = value;
            } else if range_left <= left && right <= range_right {
                self.values[node] = value;
                self.inconsistency[node] = Some(value);                        
            } else {
                self.push_inconsistency(node);
                let (left_child, right_child) = (2 * node + 1, 2 * node + 2);
                self.update_range_rec(left_child, left, (right + left) / 2, range_left, range_right, value);
                self.update_range_rec(right_child, (right + left) / 2 + 1, right, range_left, range_right, value);
                self.values[node] = self.values[left_child].max(self.values[right_child]);
            }
        }

        fn push_inconsistency(&mut self, node: usize) {
            if let Some(value) = self.inconsistency[node].take() {
                let (left_child, right_child) = (2 * node + 1, 2 * node + 2);
                if left_child >= self.n - 1 {
                    self.values[left_child] = value;
                    self.values[right_child] = value;
                } else {
                    self.inconsistency[left_child] = Some(value);
                    self.inconsistency[right_child] = Some(value);
                    self.values[left_child] = value;
                    self.values[right_child] = value;
                }
            }
        }
    }

    pub struct Solution;

    impl Solution {
        fn sorted_position_dic(positions: &Vec<Vec<i32>>) -> HashMap<i32, usize> {
            let mut coordinates = positions.iter().map(|position| [position[0], position[0] + position[1] - 1]).flatten().collect::<Vec<_>>();
            coordinates.sort_unstable();
            let mut dic = HashMap::with_capacity(positions.len() * 2);
            for coordinate in coordinates {
                if ! dic.contains_key(&coordinate) {
                    dic.insert(coordinate, dic.len());
                }
            }
            dic    
        }

        pub fn falling_squares(positions: Vec<Vec<i32>>) -> Vec<i32> {            
            let n = positions.len();
            let dic  = Self::sorted_position_dic(&positions);
            let mut tree = MaxSegmentTree::new(dic.len());
            let mut max_height = 0;
            let mut max_heights = Vec::with_capacity(n);
            for position in positions {
                let (&x1, &x2) = (dic.get(&position[0]).unwrap(), dic.get(&(position[0] + position[1] - 1)).unwrap());
                let interval_height = tree.query_range(x1, x2) + position[1] as u32;
                tree.update_range(x1, x2, interval_height);
                max_height = max_height.max(interval_height as i32);
                max_heights.push(max_height);
            }
            max_heights
        }
    }

    #[test]
    fn test_max_segment_tree() {
        let mut tree = MaxSegmentTree::new(3);
        assert_eq!(0, tree.query_range(0, 2));
        tree.update_range(2, 2, 1);
        assert_eq!(0, tree.query_range(0, 1));
        assert_eq!(1, tree.query_range(0, 2));
        tree.update_range(0, 1, 2);
        assert_eq!(2, tree.query_range(0, 1));
        assert_eq!(2, tree.query_range(0, 2));
        assert_eq!(1, tree.query_range(2, 2));
    }

    #[test]
    fn test_falling_squares() {
        assert_eq!(vec![2,5,5], Solution::falling_squares(vec![vec![1,2],vec![2,3],vec![6,1]]));
        assert_eq!(vec![100,100], Solution::falling_squares(vec![vec![100,100],vec![200,100]]));
    }
}