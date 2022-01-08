struct Solution {}

impl Solution {
    pub fn largest_rectangle_area(mut heights: Vec<i32>) -> i32 {
        let mut stack: Vec<(usize, i32)> = vec![];
        let mut max_area = 0;
        heights.push(0);
        for i in 0..heights.len() {
            let (mut j, heights_j) = (i, heights[i]);
            while stack.len() > 0 {
                let (k, heights_k) = *stack.last().unwrap();
                if heights_k > heights_j {
                    let area = (i - k) as i32 * heights_k;
                    if area > max_area { max_area = area; }
                    j = k;
                    stack.pop();
                } else if heights_k == heights_j {
                    j = k;
                    stack.pop();
                } else {
                    break;
                }
            }
            if heights_j > 0 { 
                stack.push((j, heights_j));
            }
        }
        max_area
    }
}

#[cfg(test)]
mod largest_rectangle_area_test {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::largest_rectangle_area(vec![2, 1, 5, 6, 2, 3]), 10);
    }    

    #[test]
    fn test_2() {
        assert_eq!(Solution::largest_rectangle_area(vec![2, 4]), 4);
    }    

    #[test]
    fn test_3() {
        assert_eq!(Solution::largest_rectangle_area(vec![1, 1]), 2);
    }    

    #[test]
    fn test_4() {
        assert_eq!(Solution::largest_rectangle_area(vec![2, 1, 2]), 3);
    }    
}