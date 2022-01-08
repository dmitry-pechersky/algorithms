struct Solution {}

impl Solution {
    pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
        let mut combinations: Vec<Vec<i32>> = vec![];
        fn rec(i: i32, n: i32, k: i32, combination: &mut Vec<i32>, combinations: &mut Vec<Vec<i32>>) {
            if k == 0 {
                combinations.push(combination.clone())
            } else if n - i + 1 >= k {
                rec(i + 1, n, k, combination, combinations);
                combination.push(i);
                rec(i + 1, n, k - 1, combination, combinations);
                combination.pop();
            }
        }
        rec(1, n, k, &mut vec![], &mut combinations);
        combinations
    }
}

#[cfg(test)]
mod combine_test {
    use super::*;

    #[test]
    fn test_1() {
        let mut res = Solution::combine(4, 2);
        res.sort();
        assert_eq!(res, vec![vec![1,2], vec![1,3], vec![1,4], vec![2,3], vec![2,4], vec![3,4]]);
    }    

    #[test]
    fn test_2() {
        assert_eq!(Solution::combine(1, 1), vec![vec![1]]);
    }    

}