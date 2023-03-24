struct Solution {}

impl Solution {
    pub fn find_longest_chain(pairs: Vec<Vec<i32>>) -> i32 {
        let mut pairs = pairs;
        let mut cnt = 0;
        let mut end = i32::MIN;
        pairs.sort();        
        for pair in pairs {
            if pair[0] > end {
                end = pair[1];
                cnt += 1;
            } else if pair[1] < end {
                end = pair[1];
            }
        }       
        cnt
    }
}

#[test]
fn test_1() {
    assert_eq!(2, Solution::find_longest_chain(vec![vec![1,2],vec![2,3],vec![3,4]]));
}

#[test]
fn test_2() {
    assert_eq!(3, Solution::find_longest_chain(vec![vec![1,2],vec![7,8],vec![4,5]]));
}