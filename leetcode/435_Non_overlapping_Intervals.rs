impl Solution {
    pub fn erase_overlap_intervals(intervals: Vec<Vec<i32>>) -> i32 {
        const SHIFT: i32 = 50000;
        const MAX_VALUE: usize = 100000;
        let mut begins:[i32; MAX_VALUE + 1] = [-1; MAX_VALUE + 1];
        let mut res = 0;
        for (begin, end) in intervals.into_iter().map( |interval| (interval[0] + SHIFT, (interval[1] + SHIFT) as usize)) {
            if begins[end] != -1 {
                begins[end] = begins[end].max(begin);
                res += 1;
            } else {
                begins[end] = begin;
            }
        }

        let mut max_end = -1;
        for end in 0..=MAX_VALUE {
            let begin = begins[end];
            if begin != -1 {
                if begin < max_end {
                    res += 1;
                } else {
                    max_end = end as i32;
                }
            }
        }

        res        
    }
}
struct Solution2;

impl Solution2 {
    pub fn erase_overlap_intervals(intervals: Vec<Vec<i32>>) -> i32 {
        let mut intervals = intervals;
        let mut cnt = 0;
        let mut max_end = i32::MIN;

        intervals.sort_unstable();

        for (begin, end) in intervals.into_iter().map(|interval| (interval[0], interval[1])) {
            if begin < max_end {
                max_end = max_end.min(end);
                cnt +=1;
            } else {
                max_end = end;
            }
        }
        cnt
    }
}

#[test]
fn test_1() {
    assert_eq!(Solution::erase_overlap_intervals(vec![vec![1,2],vec![2,3],vec![3,4],vec![1,3]]), 1);
}

#[test]
fn test_2() {
    assert_eq!(Solution::erase_overlap_intervals(vec![vec![1,2],vec![1,2],vec![1,2]]), 2);
}

#[test]
fn test_3() {
    assert_eq!(Solution::erase_overlap_intervals(vec![vec![1,2],vec![2,3]]), 0);
}

#[test]
fn test_solution2_1() {
    assert_eq!(Solution2::erase_overlap_intervals(vec![vec![1,2],vec![2,3],vec![3,4],vec![1,3]]), 1);
}

#[test]
fn test_solution2_2() {
    assert_eq!(Solution2::erase_overlap_intervals(vec![vec![1,2],vec![1,2],vec![1,2]]), 2);
}

#[test]
fn test_solution2_3() {
    assert_eq!(Solution2::erase_overlap_intervals(vec![vec![1,2],vec![2,3]]), 0);
}
