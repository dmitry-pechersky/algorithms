use std::collections::BinaryHeap;
use std::cmp::Reverse;

struct  Solution;

#[derive(PartialEq, Clone, Copy, Default, PartialOrd, Eq, Ord, Debug)]
struct Pos { x: i8, y: i8}

impl Pos {
    const fn new(x: i8, y: i8) -> Self {
        Pos { x, y }
    }
    fn shift(&self, delta: Pos) -> Self {
        Pos::new(self.x + delta.x, self.y + delta.y)
    }
}

struct NeighborsIter<'a> {
    grid: &'a Vec<Vec<char>>,
    bbox: Pos,
    player: Pos,
    i: usize,
}

impl<'a> NeighborsIter<'a> {
    fn new(grid: &'a Vec<Vec<char>>, bbox: Pos, player: Pos) -> Self {
        Self { grid, bbox, player, i: 0 }
    }
}

impl<'a> Iterator for NeighborsIter<'a> {
    type Item = (Pos, Pos, u16);

    fn next(&mut self) -> Option<Self::Item> {
        fn is_in_range(pos: Pos, size: Pos) -> bool {
            pos.x >= 0 && pos.x < size.x && pos.y >= 0 && pos.y < size.y
        }

        fn is_not_wall(pos: Pos, grid: &[Vec<char>]) -> bool {
            grid[pos.x as usize][pos.y as usize] != '#'
        }
        
        const DELTA: [Pos; 4] = [Pos::new(0, 1), Pos::new(1, 0), Pos::new(0, -1), Pos::new(-1, 0)];
        let size  = Pos::new(self.grid.len() as i8, self.grid[0].len() as i8);
        while self.i < 4 {
            let player = self.player.shift(DELTA[self.i]);
            self.i += 1;
            if player != self.bbox && is_in_range(player, size) && is_not_wall(player, self.grid){
                return Some((self.bbox, player, 0));
            }

        }
        if self.i < 5 {
            self.i += 1;
            for delta in DELTA {
                let (bbox, player) = (self.bbox.shift(delta), self.player.shift(delta));
                if player == self.bbox && is_in_range(bbox, size) && is_not_wall(bbox, self.grid) {
                    return Some((bbox, player, 1));
                }
            }
        }
        None
    }    
}

fn manhattan_distance(a: Pos, b: Pos) -> i8 {
    (a.x - b.x).abs() + (a.y - b.y).abs()
}

impl Solution {
    pub fn min_push_box(grid: Vec<Vec<char>>) -> i32 {
        const MAX_SIZE: usize = 20;
        let (n, m) = (grid.len(), grid[0].len());
        let (mut target, mut bbox, mut player) = (Pos::default(), Pos::default(), Pos::default());
        for x in 0..n {
            for y in 0..m {
                if grid[x][y] == 'T' {
                    target = Pos::new(x as i8, y as i8);
                } else if grid[x][y] == 'B' {
                    bbox = Pos::new(x as i8, y as i8);
                } else if grid[x][y] == 'S' {
                    player = Pos::new(x as i8, y as i8);
                }
            }
        }
        let start_heuristic = manhattan_distance(target, bbox) as u16;
        let mut heap = BinaryHeap::from([(Reverse(start_heuristic), bbox, player)]);
        let mut d = [[[[u16::MAX; MAX_SIZE]; MAX_SIZE]; MAX_SIZE]; MAX_SIZE];
        d[bbox.x as usize][bbox.y as usize][player.x as usize][player.y as usize] = 0;
        while let Some((_, bbox, player)) = heap.pop() {
            let cost = d[bbox.x as usize][bbox.y as usize][player.x as usize][player.y as usize];
            if bbox == target {
                return cost as i32;
            }
            for (next_bbox, next_player, next_weight) in NeighborsIter::new(&grid, bbox, player) {
                let next_cost = cost + next_weight;
                let next_heuristic = manhattan_distance(target, next_bbox) as u16;
                if next_cost < d[next_bbox.x as usize][next_bbox.y as usize][next_player.x as usize][next_player.y as usize]  {
                    d[next_bbox.x as usize][next_bbox.y as usize][next_player.x as usize][next_player.y as usize] = next_cost;
                    heap.push((Reverse(next_cost + next_heuristic), next_bbox, next_player));
                }
            }
        }
        -1
    }
}


#[test]
fn test_1() {
    assert_eq!(
        5,
        Solution::min_push_box(
            vec![
                vec!['#','#','#','#','#','#'],
                vec!['#','T','.','.','#','#'],
                vec!['#','.','#','B','.','#'],
                vec!['#','.','.','.','.','#'],
                vec!['#','.','.','.','S','#'],
                vec!['#','#','#','#','#','#']
            ]
        )
    );
    assert_eq!(
        7,
        Solution::min_push_box(
            vec![
                vec!['#','.','.','#','#','#','#','#'],
                vec!['#','.','.','T','#','.','.','#'],
                vec!['#','.','.','.','#','B','.','#'],
                vec!['#','.','.','.','.','.','.','#'],
                vec!['#','.','.','.','#','.','S','#'],
                vec!['#','.','.','#','#','#','#','#']
            ]        
        )
    );
}
