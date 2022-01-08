struct Solution {}

impl Solution {
    pub fn total_hamming_distance(nums: Vec<i32>) -> i32 {
        let n = nums.len(); 
        let mut cnt = 0;
        for i in 0..32 {
            let one_bit = 1 << i;
            let mut one_bit_cnt = 0;
            for num in &nums {
                one_bit_cnt += (one_bit & num) >> i;
            }
            cnt += one_bit_cnt * (n as i32 - one_bit_cnt);
        }
        cnt
    }
}

mod total_hamming_distance_test {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::total_hamming_distance(vec![4,14,2]), 6);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::total_hamming_distance(vec![4,14,4]), 4);
    }

}