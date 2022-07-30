struct Solution {}

#[derive(PartialEq)]
enum Item {
    Seat,
    Plant,
}

impl Solution {
    pub fn number_of_ways(corridor: String) -> i32 {
        let corridor = corridor.chars().map(|char| if char == 'S' { Item::Seat } else { Item::Plant } ).collect::<Vec<_>>();
        let mut first_seat = None;
        let mut last_seat = 0;
        let mut seat_cnt = 0;
        for (i, item) in corridor.iter().enumerate() {
            if *item == Item::Seat {
                seat_cnt += 1;
                last_seat = i;
                if first_seat.is_none() {
                    first_seat = Some(i);
                }
            }
        }
        if let Some(first_seat) = first_seat {
            if seat_cnt % 2 == 0 {
                let mut seat_cnt: u64 = 0;
                let mut plant_cnt: u64 = 0;
                let mut cnt: u64 = 1;
                for i in first_seat..last_seat {
                    if corridor[i] == Item::Seat {
                        seat_cnt += 1;
                        if plant_cnt > 0 {
                            cnt = cnt * (plant_cnt + 1) % 1000000007;
                            plant_cnt = 0;
                        }
                    } else {
                        if seat_cnt % 2 == 0 {
                            plant_cnt += 1;
                        }
                    }
                }
                return cnt as i32;
            }
        }
        0
    }
}

#[test]
fn test_1() {
    assert_eq!(3, Solution::number_of_ways("SSPPSPS".to_string()))
}

#[test]
fn test_2() {
    assert_eq!(1, Solution::number_of_ways("PPSPSP".to_string()))
}

#[test]
fn test_3() {
    assert_eq!(0, Solution::number_of_ways("S".to_string()))
}