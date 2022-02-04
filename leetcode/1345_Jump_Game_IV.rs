use std::collections::HashMap;
use std::collections::VecDeque;

struct Solution {}

impl Solution {
    pub fn min_jumps(arr: Vec<i32>) -> i32 {
        let n = arr.len();
        let mut dic: HashMap<i32, Vec<u32>> = HashMap::new();
        for i in 0..n {
            if let Some(indexes) = dic.get_mut(&arr[i]) {
                indexes.push(i as u32);
            } else {
                dic.insert(arr[i], vec![i as u32]);
            }
        }
        let mut queue: VecDeque<(u32, u32)> = VecDeque::new();
        let mut visited = vec![false; n];
        visited[0] = true;
        queue.push_back((0,0));
        while let Some((cost, i)) = queue.pop_front() {
            if i == n as u32 - 1 {
                return cost as i32;
            }
            if i > 0 && !visited[i as usize - 1] {
                visited[i as usize - 1] = true;
                queue.push_back((cost + 1, i - 1));
            }
            if !visited[i as usize + 1] {
                visited[i as usize + 1] = true;
                queue.push_back((cost + 1, i + 1));
            }
            if let Some(next_i_vec) = dic.remove(&arr[i as usize]) {
                for next_i in next_i_vec {
                    if !visited[next_i as usize] {
                        queue.push_back((cost + 1, next_i));
                    }
                }
            }
        }
        0
    }
}

mod min_jumps_test {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::min_jumps(vec![100,-23,-23,404,100,23,23,23,3,404]), 3);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::min_jumps(vec![7]), 0);
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::min_jumps(vec![7,6,9,6,9,6,9,7]), 1);
    }
}