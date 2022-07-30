use std::collections::BinaryHeap;

struct Solution {}

impl Solution {
    pub fn furthest_building(heights: Vec<i32>, mut bricks: i32, mut ladders: i32) -> i32 {
        let mut heap = BinaryHeap::with_capacity(heights.len());
        let mut prev_height = heights[0];
        let n = heights.len();
        for (i, height) in heights.into_iter().enumerate().skip(1) {
            if height > prev_height {
                heap.push(height - prev_height);
                bricks -= height - prev_height;
                while bricks < 0 {
                    if ladders == 0 || heap.is_empty() {
                        return i as i32 - 1;
                    }
                    let moving_height = heap.pop().unwrap();
                    bricks += moving_height;
                    ladders -= 1;
                }
            }
            prev_height = height;
        }
        n as i32 - 1
    }
}

#[test]
fn test_1() {
    assert_eq!(7, Solution::furthest_building(vec![4,12,2,7,3,18,20,3,19], 10, 2));
}

#[test]
fn test_2() {
    assert_eq!(4, Solution::furthest_building(vec![4,2,7,6,9,14,12], 5, 1));
}

#[test]
fn test_3() {
    assert_eq!(3, Solution::furthest_building(vec![14,3,19,3], 17, 0));
}