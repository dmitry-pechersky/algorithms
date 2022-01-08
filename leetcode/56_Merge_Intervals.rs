struct Solution {}

impl Solution {
    pub fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut intervals = intervals;
        let mut merged_intervals: Vec<Vec<i32>> = Vec::new();
        intervals.sort();
        let (mut current_start, mut current_end) = (intervals[0][0], intervals[0][1]);
        for interval in intervals {
            let (start, end) = (interval[0], interval[1]);
            if start <= current_end {
                if end > current_end {
                    current_end = end;
                }

            } else {
                merged_intervals.push(vec![current_start, current_end]);
                current_start = start;
                current_end = end;
            }
        }
        merged_intervals.push(vec![current_start, current_end]);
        merged_intervals
    }
}

#[cfg(test)]
mod merge_test {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            Solution::merge(vec![vec![1,3],vec![2,6],vec![8,10],vec![15,18]]), 
            vec![vec![1,6],vec![8,10],vec![15,18]]);
    }    

    #[test]
    fn test_2() {
        assert_eq!(
            Solution::merge(vec![vec![1,4],vec![4,5]]), 
            vec![vec![1,5]]);
    }    
}