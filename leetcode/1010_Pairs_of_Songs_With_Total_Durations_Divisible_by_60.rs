struct Solution {}

impl Solution {
    pub fn num_pairs_divisible_by60(time: Vec<i32>) -> i32 {
        let mut cnts = [0;60];
        let mut cnt = 0;
        for t in time {
            cnts[(t % 60) as usize] += 1;
        }
        cnt = cnts[0] * (cnts[0] - 1) / 2 +  cnts[30] * (cnts[30] - 1) / 2;
        for t in 1..30 {
            cnt += cnts[t] * cnts[60 - t];
        }
        cnt
    }
}

mod car_pooling_test {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::num_pairs_divisible_by60(vec![30,20,150,100,40]), 3);
    }    

    #[test]
    fn test_2() {
        assert_eq!(Solution::num_pairs_divisible_by60(vec![60,60,60]), 3);
    }    

}