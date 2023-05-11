use std::io;
use std::collections::BinaryHeap;
use std::cmp::Reverse;

const AL_SIZE: usize = 26;

fn count_chars(string: &String) -> [u8; AL_SIZE] {
    let mut cnts = [0; AL_SIZE];
    for byte in string.bytes() {
        cnts[(byte - b'a') as usize] += 1;
    }
    cnts
}

fn to_network(target: &String, strings: &Vec<(String, u8)>) -> (Vec<Vec<(u8,u8)>>, Vec<Vec<u8>>) {
    let network_n = 2 + AL_SIZE + strings.len();
    let mut adj_list = vec![vec![]; network_n];
    let mut capacity = vec![vec![0; network_n]; network_n];

    let target = count_chars(&target);
    for (ch, &cnt) in target.iter().enumerate() {
        let v_ch = ch + 2;
        if cnt > 0 {
            adj_list[0].push((v_ch as u8, 0));
            adj_list[v_ch ].push((0, 0));
            capacity[0][v_ch] = cnt;
        }
    }

    for (idx, (string, limit)) in strings.into_iter().enumerate() {
        let string = count_chars(&string);
        let v_string = 2 + AL_SIZE + idx;
        for (ch, &cnt) in string.iter().enumerate() {
            let v_ch = 2 + ch;
            if cnt > 0 {
                adj_list[v_ch].push((v_string as u8, 0));
                adj_list[v_string].push((v_ch as u8, 0));
                capacity[v_ch][v_string] = cnt;
            }
        }
        adj_list[v_string].push((1, idx as u8 + 1));
        adj_list[1].push((v_string as u8, 0));
        capacity[v_string][1] = *limit;
    }
    (adj_list, capacity)

}

fn bfs(source: u8, target: u8, adj_list: &Vec<Vec<(u8,u8)>>, capacity: &Vec<Vec<u8>>) -> Option<Vec<Option<u8>>> {
    let n = adj_list.len();
    let mut heap = BinaryHeap::from([(Reverse(0), source)]);
    let mut parents = vec![None; n];
    let mut costs= vec![u32::MAX; n];
    costs[source as usize] = 0;

    while let Some((Reverse(v_cost), v)) = heap.pop() {
        if v == target {
            return Some(parents);
        }
        for &(u, weight) in &adj_list[v as usize] {
            if capacity[v as usize][u as usize] > 0 {
                let u_cost = v_cost + weight as u32;
                if u_cost < costs[u as usize] {
                    costs[u as usize] = u_cost;
                    parents[u as usize] = Some(v);
                    heap.push((Reverse(u_cost), u));
                }
            }
        }
    }
    None
}

fn path_min_capacity(v: u8, parents: &Vec<Option<u8>>, capacity: &Vec<Vec<u8>>) -> u8 {
    let mut current = v;
    let mut min_capacity = u8::MAX;

    while let Some(parent) = parents[current as usize] {
        min_capacity = min_capacity.min(capacity[parent as usize][current as usize]);
        current = parent;
    }
    min_capacity
}

fn min_cost_max_flow(s: u8, t: u8, adj_list: &Vec<Vec<(u8,u8)>>, capacity: &mut Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    let n = adj_list.len();
    let mut flow = vec![vec![0; n]; n];

    while let Some(parents) = bfs(s, t, adj_list, capacity) {
        let add_flow_value = path_min_capacity(t, &parents, capacity);

        let mut current = t;
        while let Some(parent) = parents[current as usize] {
            if flow[current as usize][parent as usize] >= add_flow_value {
                flow[current as usize][parent as usize] -= add_flow_value;
            } else {
                flow[parent as usize][current as usize] += add_flow_value;
            }

            capacity[parent as usize][current as usize] -= add_flow_value;
            capacity[current as usize][parent as usize] += add_flow_value;

            current = parent;
        }
    }
    flow
}

fn build_string(target: &String, n: u8, strings: &Vec<(String, u8)>) -> i32 {
    let (adj_list, mut capacity) = to_network(&target, strings);
    let flow = min_cost_max_flow(0, 1, &adj_list, &mut capacity);

    let cost: usize = (0..n as usize).map(|i|  flow[2 + AL_SIZE + i][1] as usize * (i + 1)).sum();
    let flow_value: u8 = flow[0].iter().sum();
    if flow_value as usize != target.len() { -1 } else { cost as i32 }
}

fn main() {
    let mut iter = io::stdin().lines().flat_map(
        |line| 
        line.unwrap().split_whitespace().map(|word| word.to_string()
    ).collect::<Vec<_>>());
    let target = iter.next().unwrap();
    let n: u8 = iter.next().unwrap().parse().unwrap();
    let strings: Vec<(String, u8)> = (0..n).map(|_| (iter.next().unwrap(), iter.next().unwrap().parse().unwrap())).collect();
    
    println!("{}", build_string(&target, n, &strings))
}