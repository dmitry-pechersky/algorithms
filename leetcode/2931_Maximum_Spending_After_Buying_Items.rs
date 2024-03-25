pub struct Solution;

impl Solution {
    pub fn max_spending(values: Vec<Vec<i32>>) -> i64 {
        let (n, m) = (values.len(), values[0].len());
        let mut items = vec![m as isize - 1; n];
        let (mut total_amout, mut d) = (0, 1);
        loop {
            let (mut min_shop, mut min_value) = (-1, 0);
            for (shop, &item) in items.iter().enumerate().filter(|(_, &item)| item > -1) {
                let value = values[shop][item as usize];
                if min_shop == -1 || value < min_value {
                    min_shop = shop as isize;
                    min_value = value;
                }
            }
            if min_shop > -1 {
                total_amout += d * min_value as i64;
                d += 1;
                items[min_shop as usize] -= 1;
            } else {
                break;
            }
        }
        total_amout
    }
}

#[test]
fn test_max_spending() {
    assert_eq!(285, Solution::max_spending(vec![vec![8,5,2],vec![6,4,1],vec![9,7,3]]));
    assert_eq!(386, Solution::max_spending(vec![vec![10,8,6,4,2],vec![9,7,5,3,2]]));
}
