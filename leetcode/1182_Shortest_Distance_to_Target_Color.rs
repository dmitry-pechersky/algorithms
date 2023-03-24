struct Solution {}

impl Solution {
    pub fn shortest_distance_color(colors: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut idxs = [None, None, None];
        let mut distances = vec![[None, None, None]; colors.len()];
        for (i, &color) in colors.iter().enumerate() {
            idxs[color as usize - 1] = Some(i as u16);
            for j in 0..3 {
                distances[i][j] = idxs[j].map(|idx| i as u16 - idx);
            }
        }
        let mut idxs = [None, None, None];
        for (i, &color) in colors.iter().enumerate().rev() { 
            idxs[color as usize - 1] = Some(i as u16);
            for j in 0..3 {
                distances[i][j] = match (distances[i][j], idxs[j]) {
                    (Some(distance1), Some(idx2)) => Some(distance1.min(idx2 - i as u16)),
                    (None, Some(idx2)) => Some(idx2 - i as u16),
                    (Some(distance1), None) => Some(distance1),
                    (None, None) => None,
                };
            }
        }

        queries.iter().map(
            |query| 
            if let Some(distance) = distances[query[0] as usize][query[1] as usize - 1] {
                distance as i32
            } else {
                -1
            }
        ).collect::<Vec<_>>()
    }
}

#[test]
fn test_1() {
    assert_eq!(vec![3,0,3], Solution::shortest_distance_color(vec![1,1,2,1,3,2,2,3,3], vec![vec![1,3],vec![2,2],vec![6,1]]));
}

#[test]
fn test_2() {
    assert_eq!(vec![-1], Solution::shortest_distance_color(vec![1,2], vec![vec![0,3]]));
}
