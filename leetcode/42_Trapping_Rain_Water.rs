struct Solution {}

impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        let (mut s, mut max_height) = (0, 0);
        for i in 0..height.len() {
            if height[i] < max_height {
                s += max_height - height[i];
            } else {
                max_height = height[i];
            }
        }
        let mut i = height.len() - 1;
        let mut max_right_height = 0;
        while height[i] < max_height {
            if height[i] > max_right_height {
                max_right_height = height[i];
            }
            s -= max_height - max_right_height;
            i -= 1;
        }
        s
    }
}

#[cfg(test)]
mod trap_test {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::trap(vec![0,1,0,2,1,0,1,3,2,1,2,1]), 6);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::trap(vec![4,2,0,3,2,5]), 9);
    }
}