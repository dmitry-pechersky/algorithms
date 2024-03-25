pub mod dp_binary_search {
    use std::cmp::Reverse;

    pub struct Solution;

    impl Solution {
        fn binary_search(array: &[i32], value: i32) -> Result<usize, usize> {
            if array.last().map_or(true, |&val| val < value) {
                return Err(array.len());
            } 
            let (mut left, mut right) = (0, array.len() - 1);
            while left < right {
                let middle = (left + right) / 2;
                if array[middle] == value {
                    return  Ok(middle);
                } else if array[middle] < value {
                    left = middle + 1;
                } else {
                    right = middle;
                }
            }
            Err(right)
        }

        pub fn max_envelopes(envelopes: Vec<Vec<i32>>) -> i32 {
            let mut envelopes = envelopes;
            envelopes.sort_unstable_by_key(|envelop| (envelop[0], Reverse(envelop[1])));
            let mut dp = Vec::with_capacity(envelopes.len());
            for envelope in envelopes {
                let y = envelope[1];
                if let Err(idx) = Self::binary_search(&dp, y) {
                    if idx == dp.len() {
                        dp.push(y);
                    } else {
                        dp[idx] = y;
                    }    
                }
            }
            dp.len() as i32
        }
    }

    #[test]
    fn test_max_envelopes() {
        assert_eq!(3, Solution::max_envelopes(vec![vec![5,4],vec![6,4],vec![6,7],vec![2,3]]));
        assert_eq!(1, Solution::max_envelopes(vec![vec![1,1],vec![1,1],vec![1,1]]));
    }
}

pub mod dp {
    use std::cmp::Reverse;

    pub struct Solution;

    impl Solution {
        pub fn max_envelopes(envelopes: Vec<Vec<i32>>) -> i32 {
            let mut envelopes = envelopes;
            envelopes.sort_unstable_by_key(|envelop| (envelop[0], Reverse(envelop[1])));
            let mut dp = Vec::with_capacity(envelopes.len());
            for envelope in envelopes {
                let y = envelope[1];
                if let Err(idx) = dp.binary_search(&y) {
                    if idx == dp.len() {
                        dp.push(y);
                    } else {
                        dp[idx] = y;
                    }
                }
            }
            dp.len() as i32
        }
    }

    #[test]
    fn test_max_envelopes() {
        assert_eq!(3, Solution::max_envelopes(vec![vec![5,4],vec![6,4],vec![6,7],vec![2,3]]));
        assert_eq!(1, Solution::max_envelopes(vec![vec![1,1],vec![1,1],vec![1,1]]));
    }
}