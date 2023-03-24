struct Solution {}

impl Solution {
    pub fn min_meeting_rooms(intervals: Vec<Vec<i32>>) -> i32 {
        let mut starts = intervals.iter().map(|v| v[0]).collect::<Vec<_>>();
        let mut ends = intervals.iter().map(|v| v[1]).collect::<Vec<_>>();
        starts.sort();
        ends.sort();
        let (mut s, mut e) = (0, 0);
        let (mut cnt, mut max_cnt) = (0, 0);
        while s < starts.len() {
            let day = starts[s].min(ends[e]);
            while s < starts.len() && starts[s] == day {
                cnt += 1;
                s += 1;
            }
            while e < ends.len() && ends[e] == day {
                cnt -= 1;
                e += 1;
            }
            max_cnt = max_cnt.max(cnt);
        }
        max_cnt
    }
}

#[test]
fn test_1() {
    assert_eq!(
        2,
        Solution::min_meeting_rooms(vec![vec![0,30],vec![5,10],vec![15,20]])
    );
}

#[test]
fn test_2() {
    assert_eq!(
        1,
        Solution::min_meeting_rooms(vec![vec![7,10],vec![2,4]])
    );
}
