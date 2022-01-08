struct Solution {}

impl Solution {
    pub fn is_power_of_two(n: i32) -> bool {
        n > 0 && n.count_ones() == 1        
    }
}

mod test_is_power_of_two {
    use super::*;

    #[test]
    fn test_1() {
        assert!(Solution::is_power_of_two(1));
    }

    #[test]
    fn test_2() {
        assert!(Solution::is_power_of_two(16));
    }

    #[test]
    fn test_3() {
        assert!(! Solution::is_power_of_two(3));
    }

}