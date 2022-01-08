struct Solution {}

impl Solution {
    pub fn car_pooling(trips: Vec<Vec<i32>>, capacity: i32) -> bool {
        let mut days = vec![];
        for trip in &trips {
            days.push((trip[1], trip[0]));
            days.push((trip[2], -trip[0]));
        }   
        days.sort();
        let mut current_num = 0;
        let mut current_day = 0;
        for (day, num) in days {
            if current_day != day {
                if current_num > capacity {
                    return false;
                }
                current_day = day;
            }
            current_num += num;
        }
        current_num <= capacity
    }
}

mod car_pooling_test {
    use super::*;

    #[test]
    fn test_1() {
        assert!(!Solution::car_pooling(vec![vec![2,1,5], vec![3,3,7]], 4));
    }    

    #[test]
    fn test_2() {
        assert!(Solution::car_pooling(vec![vec![2,1,5], vec![3,3,7]], 5));
    }    

}