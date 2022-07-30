struct Solution {}

#[derive(Clone, PartialEq)]
enum State {
    NotVisited,
    InStack,
    Done,
}

impl Solution {
    pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
        let n = num_courses as usize;
        let mut adj_list = vec![vec![]; n];
        for (v, u) in prerequisites.into_iter().map(|edge| (edge[0] as usize, edge[1] as usize)){
            adj_list[u].push(v);
        }
        let mut stack: Vec<usize>= Vec::with_capacity(n);
        let mut states = vec![State::NotVisited; n];
        for v in 0..n {
            if states[v] == State::NotVisited {
                stack.push(v);
                while let Some(v) = stack.pop() {
                    match states[v] {
                        State::NotVisited => {
                            states[v] = State::InStack;
                            stack.push(v);
                            for &u in adj_list[v].iter() {
                                match states[u] {
                                    State::InStack => { return false; },
                                    State::NotVisited => { stack.push(u); },
                                    State::Done => {}
                                }
                            }
                        },
                        State::InStack => { states[v] = State::Done; },
                        State::Done => {},
                    };
                }
            }            
        }
        true
    }
}

#[test]
fn test_1() {
    assert!(Solution::can_finish(2, vec![vec![1,0]]));
}

#[test]
fn test_2() {
    assert!(! Solution::can_finish(2, vec![vec![1,0],vec![0,1]]));
}
