struct Solution {}

impl Solution {
    pub fn minimum_total(triangle: Vec<Vec<i32>>) -> i32 {
        let mut triangle = triangle;
        for i in 1..triangle.len() {
            let n = triangle[i].len();
            triangle[i][0] += triangle[i - 1][0];
            triangle[i][n - 1] += triangle[i - 1][n - 2];
            for j in 1..n - 1 {
                triangle[i][j] += triangle[i - 1][j].min(triangle[i - 1][j - 1]);
            }
        }
        *triangle.last().unwrap().iter().min().unwrap()
    }
}


#[test] 
fn test_1() {
    assert_eq!(11, Solution::minimum_total(vec![vec![2],vec![3,4],vec![6,5,7],vec![4,1,8,3]]));
}

#[test] 
fn test_2() {
    assert_eq!(-10, Solution::minimum_total(vec![vec![-10]]));
}