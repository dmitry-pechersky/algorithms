struct Solution {}

impl Solution {
    pub fn winner_square_game(n: i32) -> bool {
        let squares: Vec<u32> = (1..(n as f64).sqrt() as u32 + 1).map(|x| x * x).collect();
        let mut dp = vec![false; n as usize + 1];
        for i in 1..(n as u32 + 1) {
            for square in &squares {
                if *square > i {
                    break
                } else if ! dp[(i - *square) as usize] {
                    dp[i as usize] = true;
                    break
                }
            }
        }
        dp[n as usize]
    }
}


mod winner_square_game_test {
    use super::*;

    #[test]
    fn test_1() {
        assert!(Solution::winner_square_game(1));
    }

    #[test]
    fn test_2() {
        assert!(!Solution::winner_square_game(2));
    }
    
    #[test]
    fn test_3() {
        assert!(Solution::winner_square_game(4));
    }
}