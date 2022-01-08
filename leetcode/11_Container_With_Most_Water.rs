struct Solution { }

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let (mut i, mut j) = (0, height.len() - 1);
        let mut area = 0;
        while i < j {
            area = area.max((j - i) as i32 * height[i].min(height[j]));
            if height[i] < height[j] {
                i += 1;
            } else {
                j -= 1;
            }
        }
        area
    }
}

#[cfg(test)]
mod max_area_test {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::max_area(vec![1,8,6,2,5,4,8,3,7]), 49);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::max_area(vec![4,3,2,1,4]), 16);
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::max_area(vec![1,2,1]), 2);
    }

}