struct Solution {}

impl Solution {
    pub fn hamming_distance(x: i32, y: i32) -> i32 {
        (x ^ y).count_ones() as i32
    }
}
 
mod hamming_distance_test {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::hamming_distance(1, 4), 2);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::hamming_distance(3, 1), 1);
    }

}
