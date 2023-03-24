struct Solution {}

impl Solution {
    pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
        let mut res = vec![0;temperatures.len()];
        let mut stack = vec![(0, temperatures[0])];
        for (i, temp) in temperatures.into_iter().enumerate().skip(1) {
            while let Some((prev_i, prev_temp)) = stack.pop() {
                if prev_temp < temp {
                    res[prev_i] = (i - prev_i) as i32;
                } else {
                    stack.push((prev_i, prev_temp));
                    break;
                }
            }
            stack.push((i, temp));
        }
        res
    }
}

#[test]
fn temp_1() {
    assert_eq!(vec![1,1,4,2,1,1,0,0], Solution::daily_temperatures(vec![73,74,75,71,69,72,76,73]));
}

#[test]
fn temp_2() {
    assert_eq!(vec![1,1,1,0], Solution::daily_temperatures(vec![30,40,50,60]));
}

#[test]
fn temp_3() {
    assert_eq!(vec![1,1,0], Solution::daily_temperatures(vec![30,60,90]));
}
