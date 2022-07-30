use std::collections::BinaryHeap;

struct Solution {}

impl Solution {
    pub fn minimum_effort_path(heights: Vec<Vec<i32>>) -> i32 {
        let (rows, columns) = (heights.len(), heights[0].len());
        let mut heap = BinaryHeap::new();
        let mut expanded = vec![vec![i32::MIN; columns]; rows];
        expanded[0][0] = 0;
        heap.push((0,0,0));
        while let Some((cost, x, y)) = heap.pop() {
            if x == rows - 1 && y == columns - 1 {
                return -cost;
            }
            if  cost >= expanded[x][y] {
                for (x1, y1) in [(1, 0), (0, 1), (-1, 0), (0, -1)].iter()
                                    .map(|(dx, dy)| (x as i32 + dx, y as i32 + dy))
                                    .filter(|(x1, y1)| 0 <= *x1 && *x1 < rows as i32 && 0 <= *y1 && *y1 < columns as i32)
                                    .map(|(x1, y1)| (x1 as usize, y1 as usize)) 
                {
                    let move_cost = - (heights[x][y] - heights[x1][y1]).abs();
                    let cost1 = if move_cost < cost { move_cost } else { cost }; 
                    if cost1 > expanded[x1][y1]  {
                        expanded[x1][y1] = cost1;
                        heap.push((cost1 ,x1, y1,));
                    }
                }
            }
        }
        unreachable!();
    }
}

#[test]
fn test_1() {
    assert_eq!(2, Solution::minimum_effort_path(vec![vec![1,2,2], vec![3,8,2], vec![5,3,5]]));
}

#[test]
fn test_2() {
    assert_eq!(1, Solution::minimum_effort_path(vec![vec![1,2,3], vec![3,8,4], vec![5,3,5]]))
}

#[test]
fn test_3() {
    assert_eq!(0, Solution::minimum_effort_path(vec![vec![1,2,1,1,1],vec![1,2,1,2,1],vec![1,2,1,2,1],vec![1,2,1,2,1],vec![1,1,1,2,1]]));
}

#[test]
fn test_4() {
    assert_eq!(9, Solution::minimum_effort_path(vec![vec![1,10,6,7,9,10,4,9]]));
}