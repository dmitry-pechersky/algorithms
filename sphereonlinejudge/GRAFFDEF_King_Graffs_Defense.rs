use std::{io};

#[derive(Clone, Debug)]
enum DFSColor { White, Grey, Black }

fn edges_to_adj_list(n: u32, edges: &[(u32, u32)]) -> Vec<Vec<u32>> {
    let mut adj_list = vec![vec![]; n as usize];
    for &(v, u) in edges {
        adj_list[v as usize].push(u);
        adj_list[u as usize].push(v);
    }
    adj_list
}

fn biconnected_components(adj_list: &Vec<Vec<u32>>) -> Vec<u32> {
    fn dfs(
        v: u32, adj_list: &Vec<Vec<u32>>, colors: &mut [DFSColor], parents: &mut [u32], 
        time: &mut u32, in_times: &mut [u32], ret_times: &mut [u32], stack: &mut Vec<u32>, component_sizes: &mut Vec<u32>
    ) {
        colors[v as usize] = DFSColor::Grey;
        in_times[v as usize] = *time;
        ret_times[v as usize] = *time;
        *time += 1;
        stack.push(v);
        for &u in &adj_list[v as usize] {
            if u != parents[v as usize] {
                match colors[u as usize] {
                    DFSColor::White => {
                        parents[u as usize] = v;
                        dfs(u, adj_list, colors, parents, time, in_times, ret_times, stack, component_sizes);
                        ret_times[v as usize] = ret_times[v as usize].min(ret_times[u as usize]);
                    },
                    DFSColor::Grey => {
                        ret_times[v as usize] = ret_times[v as usize].min(in_times[u as usize]);
                    },
                    DFSColor::Black => {}
                }
            }
        }        
        if in_times[v as usize] == ret_times[v as usize] {
            let mut component_size = 1;
            while Some(v) != stack.pop() {
                component_size += 1;
            }
            component_sizes.push(component_size);
        }
        colors[v as usize] = DFSColor::Black;
    }

    let n = adj_list.len();
    let mut colors = vec![DFSColor::White; n];
    let mut parents: Vec<u32> = (0..n as u32).collect();
    let mut time = 0;
    let mut in_times = vec![0; n];
    let mut ret_times = vec![0; n];
    let mut stack = vec![];
    let mut component_sizes = vec![];

    for v in 0..n as u32 {
        match colors[v as usize] {
            DFSColor::White => {
                dfs(v, adj_list, &mut colors, &mut parents, &mut time, &mut in_times, &mut ret_times, &mut stack, &mut component_sizes);
            },
            _ => { }
        }
    }
    component_sizes
}

fn read_number_pair() -> (u32, u32) {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    let mut iter = line.trim().split(' ').map(|value| value.parse().unwrap());
    (iter.next().unwrap(), iter.next().unwrap())
}

fn main() {
    let (n, m) = read_number_pair();
    let edges: Vec<(u32, u32)> = (0..m).map(|_| { let (v, u) = read_number_pair(); (v - 1, u - 1) }).collect();
    let component_sizes = biconnected_components(&edges_to_adj_list(n, &edges));
    let succes_combinations: u64 = component_sizes.iter().filter(|&&size| size > 1).map(|&size| size as u64 * (size as u64 - 1) / 2).sum();
    let total_combinations = n as u64 * (n  as u64 - 1) / 2;
    println!("{:.1$}", (total_combinations - succes_combinations) as f64 / total_combinations as f64, 5);
}
