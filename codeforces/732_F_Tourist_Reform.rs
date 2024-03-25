use std::{io::{stdin, Read}, fmt::Write};

#[derive(Clone)]
enum DFSColor { White, Grey, Black }

fn read_input() -> (usize, Vec<Vec<(usize, usize)>>) {
    let mut buffer = String::new();
    stdin().read_to_string(&mut buffer).unwrap();
    let mut input = buffer.trim().split_ascii_whitespace().map(|word| word.parse::<usize>().unwrap());
    let (n, m) = (input.next().unwrap(), input.next().unwrap());
    let mut adj_list = vec![vec![]; n];
    for idx in 0..m {
        let (v, u) = (input.next().unwrap(), input.next().unwrap());
        adj_list[v - 1].push((u - 1, idx));
        adj_list[u - 1].push((v - 1, idx));
    }
    (m, adj_list)
}

fn write_ouput(largest_bcc_size: usize, edges: &[(usize, usize)]) {
    let mut buffer = String::new();
    writeln!(buffer, "{}", largest_bcc_size).unwrap();
    for (v, u) in edges {
        writeln!(buffer, "{} {}", v + 1, u + 1).unwrap();
    }
    println!("{}", buffer);
}

fn largest_biconnected_component_node(adj_list: &[Vec<(usize, usize)>]) -> (usize, usize) {
    fn dfs(
        v: usize, adj_list: &[Vec<(usize, usize)>], parent: usize, colors: &mut [DFSColor], 
        t: usize, t_in: &mut [usize], ret: &mut [usize],
        largest_bcc_size: &mut usize, largest_bcc_node: &mut usize
    ) -> usize 
    {
        colors[v] = DFSColor::Grey;
        t_in[v] = t;
        ret[v] = t;
        let mut v_bcc_size = 1;
        for &(u, _) in adj_list[v].iter().filter(|(u, _)| *u != parent) {
            match colors[u] {
                DFSColor::White => {
                    let u_bcc_size = dfs(u, adj_list, v, colors, t + 1, t_in, ret, largest_bcc_size, largest_bcc_node);
                    ret[v] = ret[v].min(ret[u]);
                    if t_in[v] < ret[u] {
                        if u_bcc_size > *largest_bcc_size {
                            *largest_bcc_size = u_bcc_size;
                            *largest_bcc_node = u;
                        }
                    } else {
                        v_bcc_size += u_bcc_size;
                    }
                },
                DFSColor::Grey => {
                    ret[v] = ret[v].min(t_in[u]);
                },
                DFSColor::Black => {}
            }
        }
        colors[v] = DFSColor::Black;
        v_bcc_size
    }

    let n = adj_list.len();
    let mut colors = vec![DFSColor::White; n];
    let mut t_in = vec![0; n];
    let mut ret = vec![0; n];
    let (mut largest_bcc_size, mut largest_bcc_node) = (0, 0);
    
    let bcc_size = dfs(0, adj_list, 0, &mut colors, 0, &mut t_in, &mut ret, &mut largest_bcc_size, &mut largest_bcc_node);    
    if bcc_size > largest_bcc_size {
        (bcc_size, 0)
    } else {
        (largest_bcc_size, largest_bcc_node)
    }
}

fn orient_edges(start: usize, adj_list: &[Vec<(usize, usize)>], m: usize) -> Vec<(usize, usize)> {
    fn dfs(v: usize, adj_list: &[Vec<(usize, usize)>], parent: usize, colors: &mut [DFSColor], edges: &mut [(usize, usize)]) {
        colors[v] = DFSColor::Grey;
        for &(u, edge_idx) in &adj_list[v] {
            if u != parent {
                match  colors[u] {
                    DFSColor::White => {
                        dfs(u, adj_list, v, colors, edges);
                        edges[edge_idx] = (u, v);
                    }, 
                    DFSColor::Grey => {
                        edges[edge_idx] = (u, v);
                    },
                    DFSColor::Black => {}
                } 
            }
        }
        colors[v] = DFSColor::Black;
    }

    let n =  adj_list.len();
    let mut colors = vec![DFSColor::White; n];
    let mut edges = vec![(0,0); m];
    dfs(start, adj_list, start, &mut colors, &mut edges);
    edges
}

fn main() {    
    let (m, adj_list) = read_input();
    let (largest_bcc_size, largest_bcc_node) = largest_biconnected_component_node(&adj_list);
    write_ouput(largest_bcc_size, &orient_edges(largest_bcc_node, &adj_list, m));
}