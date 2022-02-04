struct Solution {}

impl Solution {
    pub fn is_robot_bounded(instructions: String) -> bool {
        fn next(state: (i32, i32, i32, i32), action: char) -> (i32, i32, i32, i32) {
            let (x, y, dx, dy) = state;
            match action {
                'L' => (x, y, dy, -dx),
                'R' => (x, y, -dy, dx),
                _ => (x + dx, y + dy, dx, dy),
            }
        }
        let mut state = (0, 0, 1, 0);
        for action in instructions.chars() {
            state = next(state, action);
        }
        let (x, y, dx, dy) = state;
        (x == 0 && y == 0) || !(dx == 1 && dy == 0)
    }
}

mod is_robot_bounded_test {
    use super::*;

    #[test]
    fn test_1() {
        assert!(Solution::is_robot_bounded("GGLLGG".to_string()));
    }    

    #[test]
    fn test_2() {
        assert!(!Solution::is_robot_bounded("GG".to_string()));
    }    

    #[test]
    fn test_3() {
        assert!(Solution::is_robot_bounded("GL".to_string()));
    }    

}