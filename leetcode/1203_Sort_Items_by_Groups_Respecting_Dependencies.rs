pub struct Solution;

#[derive(Clone, PartialEq, Debug)]
enum Color { White, Grey, Black }

impl Solution {
    fn group(mut m: usize, group: Vec<i32>) -> (Vec<u16>, Vec<Vec<u16>>) {
        let groups = group.into_iter().map(
            |group| 
            if group == -1 {
                m += 1;
                m as u16 - 1
            } else {
                group as u16
            }
        ).collect::<Vec<u16>>();

        let mut group_nodes = vec![vec![]; m];
        for (v, &group) in  groups.iter().enumerate() { 
            group_nodes[group as usize].push(v as u16);
        }

        (groups, group_nodes)
    }

    fn adj_lists(n: usize, m: usize, groups: &[u16], before_items: Vec<Vec<i32>>) -> (Vec<Vec<u16>>,Vec<Vec<u16>>) {
        let mut adj_list = vec![vec![]; n];
        let mut group_adj_list = vec![vec![]; m];

        for (v, items) in before_items.into_iter().enumerate() {
            for u in items {
                if groups[v] == groups[u as usize] {
                    adj_list[v].push(u as u16);
                } else {
                    group_adj_list[groups[v] as usize].push(groups[u as usize]);
                }
            }
        }
        (adj_list, group_adj_list)
    }

    fn toposort(nodes: impl Iterator<Item = u16>, 
        adj_list: &Vec<Vec<u16>>, colors: &mut Vec<Color>, stack: &mut Vec<u16>, order: &mut Vec<u16>) -> bool 
    {
        for w in nodes {
            if colors[w as usize] == Color::White {
                stack.push(w as u16);
                while let Some(v) = stack.pop() {
                    match colors[v as usize] {
                        Color::White => {
                            colors[v as usize] = Color::Grey;
                            stack.push(v);
                            for &u in &adj_list[v as usize] {
                                match colors[u as usize] {
                                    Color::White => {
                                        stack.push(u);
                                    },
                                    Color::Grey => {
                                        return false;
                                    },
                                    Color::Black => {}
                                }
                            }
                        },
                        Color::Grey => {
                            colors[v as usize] = Color::Black;
                            order.push(v)
                        }, 
                        Color::Black => {
                        }            
                    }
                }
            } 
        }
        true
    }

    pub fn sort_items(n: i32, m: i32, group: Vec<i32>, before_items: Vec<Vec<i32>>) -> Vec<i32> {
        let n = n as usize;
        let (groups, group_nodes) = Self::group(m as usize, group);

        let m = group_nodes.len();

        let (adj_list, group_adj_list) = Self::adj_lists(n, m, &groups, before_items);

        let mut colors = vec![Color::White; n];
        let mut group_colors = vec![Color::White; m];
        let mut order = Vec::with_capacity(n);
        let mut group_order = Vec::with_capacity(m);
        let mut stack = vec![];

        if Self::toposort(0..m as u16, &group_adj_list, &mut group_colors, &mut stack, &mut group_order) {
            for group in group_order {
                if ! Self::toposort(group_nodes[group as usize].iter().copied(), &adj_list, &mut colors, &mut stack, &mut order) {
                    return vec![];
                }    
            }
        } else {
            return vec![]
        }
        order.into_iter().map(|value| value as i32).collect()
    }
}
