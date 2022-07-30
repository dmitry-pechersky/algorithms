struct Solution {}

impl Solution {
    pub fn combination_sum3(k: i32, n: i32) -> Vec<Vec<i32>> {
        fn rec(start: i32, k: i32, n: i32, combination: &mut Vec<i32>, combinations: &mut Vec<Vec<i32>>) {
            if k == 0 && n == 0 {
                combinations.push(combination.clone());
            } else if k > 0 && n > 0 {
                for i in start..=9 {
                    combination.push(i);
                    rec(i + 1, k - 1, n - i, combination, combinations);
                    combination.pop();
                }
            }
        }

        let mut combination = vec![];
        let mut combinations = vec![];
        rec(1, k, n, &mut combination, &mut combinations);
        combinations
    }
}

#[test]
fn test_1() {
    assert_eq!(vec![vec![1,2,4]], Solution::combination_sum3(3, 7));
}

#[test]
fn test_2() {
    assert_eq!(vec![vec![1,2,6],vec![1,3,5],vec![2,3,4]], Solution::combination_sum3(3, 9));
}

#[test]
fn test_3() {
    assert_eq!(Vec::<Vec<i32>>::new(), Solution::combination_sum3(4, 1));
}
