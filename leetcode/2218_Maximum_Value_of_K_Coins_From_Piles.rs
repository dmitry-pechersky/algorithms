pub struct Solution;

impl Solution {
    pub fn max_value_of_coins(piles: Vec<Vec<i32>>, k: i32) -> i32 {
        let k = k as usize;
        let mut piles = piles;
        let n = piles.len();


        for i in 0..n {
            for j in 1..piles[i].len() {
                piles[i][j] += piles[i][j - 1];
            }
        }    

        let mut dp_1 = vec![0; k + 1];
        let mut dp_2 = vec![0; k + 1];

        for i in 0..n {
            for j in 0..k + 1 {
                dp_2[j] = dp_1[j].max(dp_2[j]);
                for l in 0..piles[i].len().min(k - j) {
                    dp_2[j + l + 1] = dp_2[j + l + 1].max(dp_1[j] + piles[i][l]);
                }
            }
            std::mem::swap(&mut dp_1, &mut dp_2);
        }
        dp_1[k]
    }
}

#[test]
fn max_value_of_coins_test() {
    assert_eq!(101, Solution::max_value_of_coins(vec![vec![1,100,3],vec![7,8,9]], 2));
    assert_eq!(
        706, 
        Solution::max_value_of_coins(
            vec![vec![100],vec![100],vec![100],vec![100],vec![100],vec![100],vec![1,1,1,1,1,1,700]], 
            7
        )
    );
    assert_eq!(
        494, 
        Solution::max_value_of_coins(
            vec![vec![37,88],vec![51,64,65,20,95,30,26],vec![9,62,20],vec![44]], 
            9
        )
    );

}