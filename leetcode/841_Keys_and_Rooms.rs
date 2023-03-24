struct Solution {}

impl Solution {
    pub fn can_visit_all_rooms(rooms: Vec<Vec<i32>>) -> bool {
        let n = rooms.len();
        let mut visited = vec![false; n];
        visited[0] = true;
        let mut stack = vec![0];
        while let Some(room) = stack.pop() {
            for &next_room in rooms[room].iter() {
                if ! visited[next_room as usize] {
                    visited[next_room as usize] = true;
                    stack.push(next_room as usize);
                }
            }
        }
        visited.into_iter().all(|v| v)
    }
}

#[test]
fn test_1() {
    assert!(Solution::can_visit_all_rooms(vec![vec![1],vec![2],vec![3],vec![]]));
}

#[test]
fn test_2() {
    assert!(!Solution::can_visit_all_rooms(vec![vec![1,3],vec![3,0,1],vec![2],vec![0]]));
}
