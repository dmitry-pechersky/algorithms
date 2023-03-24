struct Solution {}

impl Solution {
    pub fn min_swaps(data: Vec<i32>) -> i32 {
        let one_cnt = data.iter().filter(|&v| *v == 1).count();
        let mut window_one_cnt = data[0..one_cnt].iter().filter(|&v| *v == 1).count();
        let mut window_max_one_cnt = window_one_cnt;
        for right in one_cnt..data.len() {
            if data[right - one_cnt] == 1 {
                window_one_cnt -= 1;
            }
            if data[right] == 1 {
                window_one_cnt += 1;
            }
            window_max_one_cnt =window_max_one_cnt.max(window_one_cnt);
        }
        (one_cnt - window_max_one_cnt) as i32
    }
}

#[test]
fn test_1() {
    assert_eq!(1, Solution::min_swaps(vec![1,0,1,0,1]));
}

#[test]
fn test_2() {
    assert_eq!(0, Solution::min_swaps(vec![0,0,0,1,0]));
}

#[test]
fn test_3() {
    assert_eq!(3, Solution::min_swaps(vec![1,0,1,0,1,0,0,1,1,0,1]));
}