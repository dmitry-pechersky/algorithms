use std::{collections::{VecDeque, HashSet}, hash::Hash};
struct Solution {}

#[derive(PartialEq)]
enum GCell { Empty, Wall, Start, Key(u8), Lock(u8), } 

impl GCell {
    fn new(cell: char) -> Self {
        match cell {
            '.' => { Self::Empty },
            '#' => { Self::Wall },
            '@' => { Self::Start },
            'a'..='z' => { Self::Key(cell as u8 - 'a' as u8) },
            'A'..='Z' => { Self::Lock(cell as u8 - 'A' as u8) },
            _ => { unreachable!("Wrong grid cell value") }
        }
    }
}
#[derive(Eq, Hash, PartialEq, Clone, Copy, Debug)]
struct State { x: u8, y: u8, keys: u32 }

struct Successor<'a> { state: State, i: usize, grid:  &'a Vec<Vec<GCell>>, n: usize, m: usize }

impl State {
    fn successors(self, grid: &Vec<Vec<GCell>>) -> Successor {
        Successor { state: self, i: 0, grid: grid, n: grid.len(), m: grid[0].len()}
    }

    fn has_key(&self, key: u8) -> bool {
        1 << key & self.keys != 0 
    }
}

impl<'a> Iterator for Successor<'a> {
    type Item = State;

    fn next(&mut self) -> Option<Self::Item> {
        let dxdy = [(0, 1), (1, 0), (0, -1), (-1, 0)];
        while self.i < dxdy.len() {
            let (dx, dy) = dxdy[self.i];
            let (x, y) = (self.state.x as i32 + dx, self.state.y as i32 + dy);
            self.i += 1;
            if 0 <= x && x < self.n as i32 && 0 <= y && y < self.m as i32 {
                let (x, y) = (x as u8, y as u8);
                match self.grid[x as usize][y as usize] {
                    GCell::Empty | GCell::Start => {
                        return Some(State { x, y, keys: self.state.keys });
                    },
                    GCell::Lock(key) if self.state.has_key(key) => {
                        return Some(State { x, y, keys: self.state.keys });
                    },
                    GCell::Key(key) => {
                        return Some(State { x, y, keys: 1 << key | self.state.keys });
                    },
                    _ => {},
                }
            }
        }
        None
    }
}

impl Solution {
    pub fn shortest_path_all_keys(grid: Vec<String>) -> i32 {
        let grid: Vec<Vec<GCell>> = grid.into_iter().map( |row| row.chars().map(|ch| GCell::new(ch)).collect::<_>()).collect::<_>();
        let(n, m) = (grid.len(), grid[0].len());
        let mut all_keys = 0u32;
        let mut start = State {x: 0, y: 0, keys: 0 };
        for x in 0..n {
            for y in 0..m {
                match grid[x][y] {
                    GCell::Key(key) => { all_keys = 1 << key | all_keys; },
                    GCell::Start => { start = State { x: x as u8, y: y as u8, keys: 0 }; },
                    _ => {},
                }
            }
        }

        let mut queue = VecDeque::from([(0, start)]);
        let mut expanded: HashSet<State>  = HashSet::from([start]);
        while let Some((cost, state)) = queue.pop_front() {
            if state.keys == all_keys {
                return cost;
            } else {
                for successor in state.successors(&grid) {
                    if ! expanded.contains(&successor){
                        queue.push_back((cost + 1, successor));
                        expanded.insert(successor);
                    }
                }                    
            }
        }
        -1
    }
}

#[test]
fn test_1() {
    assert_eq!(8, Solution::shortest_path_all_keys(vec!["@.a.#".to_string(), "###.#".to_string() ,"b.A.B".to_string()]));
}

#[test]
fn test_2() {
    assert_eq!(6, Solution::shortest_path_all_keys(vec!["@..aA".to_string(), "..B#.".to_string() ,"....b".to_string()]));
}

#[test]
fn test_3() {
    assert_eq!(-1, Solution::shortest_path_all_keys(vec!["@Aa".to_string()]));
}

#[test]
fn test_4() {
    assert_eq!(10, Solution::shortest_path_all_keys(vec!["@...a".to_string(), ".###A".to_string(), "b.BCc".to_string()]));
}
