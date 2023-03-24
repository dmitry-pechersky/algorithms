use std::collections::HashSet;

struct Solution {}

impl Solution {
    pub fn k_closest(points: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        fn partition(array: &mut Vec<(usize, i32)>, left: usize, right: usize, partition_idx: usize) -> usize {
            let partition_value = array[partition_idx].1;
            array.swap(right, partition_idx);
            let mut idx = left;
            for i in left..right {
                if array[i].1 < partition_value {
                    array.swap(i, idx);
                    idx += 1;
                }
            }
            array.swap(idx, right);
            idx
        }

        fn quick_select(array: &mut Vec<(usize, i32)>, k: usize) {
            let (mut left, mut right) = (0, array.len() - 1);
            while left < right {
                let partition_idx = left + (right - left) / 2;
                let idx = partition(array, left, right, partition_idx);
                if idx == k {
                    break;
                } else if idx < k {
                    left = idx + 1;
                } else {
                    right = idx - 1;
                }
            }
        }

        let mut distances = points.iter().enumerate().map(|(i, v)| (i, v[0] * v[0] + v[1] * v[1])).collect::<Vec<_>>();
        quick_select(&mut distances, k as usize);
        distances[0..k as usize].iter().map(|(i, _)| points[*i].clone()).collect::<Vec<_>>()
    }
}

#[test]
fn test_1() {
    assert_eq!(
        vec![vec![-2,2]],
        Solution::k_closest(vec![vec![1,3],vec![-2,2]], 1)
    );
}

#[test]
fn test_2() {
    assert_eq!(
        HashSet::from([vec![3,3],vec![-2,4]]),
        Solution::k_closest(vec![vec![3,3],vec![5,-1],vec![-2,4]], 2).into_iter().collect::<HashSet<_>>()
    );
}
