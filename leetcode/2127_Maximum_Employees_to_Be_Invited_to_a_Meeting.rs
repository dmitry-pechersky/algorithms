pub struct Solution;

#[derive(Clone, Copy, PartialEq)]
enum Color {White, Grey, Black}

fn strongly_connected_components(edges: &[i32]) -> Vec<Vec<u32>> {
    let n = edges.len();
    let mut stack = vec![];
    let mut colors = vec![Color::White; n];
    let mut components = vec![];
    for v in 0..n {
        if colors[v] == Color::White {
            colors[v] = Color::Grey;
            stack.push(v);

            loop {
                let u = edges[*stack.last().unwrap()] as usize;
                match colors[u] {
                    Color::White => {
                        colors[u] = Color::Grey;
                        stack.push(u);
                    }, 
                    Color::Grey => {
                        let mut component = vec![];
                        while let Some(w) = stack.pop() {
                            component.push(w as u32);
                            colors[w] = Color::Black;
                            if w == u {
                                break
                            }
                        }
                        components.push(component);
                        break;
                    }, 
                    Color::Black => {
                        break;
                    }
                }
            }

            while let Some(w) = stack.pop() {
                colors[w] = Color::Black;
            }
        }
    }
    components
}

fn reverse(edges: &Vec<i32>) -> Vec<Vec<u32>> {
    let n = edges.len();
    let mut adj_list = vec![vec![]; n];
    for (v, &u) in edges.iter().enumerate() {
        adj_list[u as usize].push(v as u32);
    }
    adj_list
}

impl Solution {
    pub fn maximum_invitations(favorite: Vec<i32>) -> i32 {        
        let components = strongly_connected_components(&favorite);
        let adj_list = reverse(&favorite);
        let max_size = components.iter().map(|component| component.len()).max().unwrap_or(1);

        let mut stack = vec![];
        let mut total_two_length = 0;
        for component in components {
            if component.len() == 2 {
                let mut two_length = 2;
                for &v in &component {
                    let mut max_length = 0;
                    for &u in &adj_list[v as usize] {
                        if !component.contains(&u) {
                            stack.push((1,u));
                            while  let Some((length, w)) = stack.pop() {
                                max_length = max_length.max(length);
                                for &k in &adj_list[w as usize] {
                                    stack.push((length + 1, k));                                    
                                }
                            }
                        }
                    }
                    two_length += max_length;
                }
                total_two_length += two_length;
            }
        }

        max_size.max(total_two_length) as i32
    }
}

#[test]
fn test_1() {
    assert_eq!(3, Solution::maximum_invitations(vec![2,2,1,2]));
    assert_eq!(3, Solution::maximum_invitations(vec![1,2,0]));
    assert_eq!(4, Solution::maximum_invitations(vec![3,0,1,4,1]));
    assert_eq!(11, Solution::maximum_invitations(vec![1,0,3,2,5,6,7,4,9,8,11,10,11,12,10]));
}