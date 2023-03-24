use std::{io};

fn edges_to_adj_list(n: u32, edges: &[(u32, u32)]) -> Vec<Vec<u32>> {
    let mut adj_list = vec![vec![]; n as usize];
    for &(v, u) in edges {
        adj_list[v as usize].push(u);
    }
    adj_list
}

fn transpose(adj_list: &Vec<Vec<u32>>) -> Vec<Vec<u32>> {
    let mut transposed_adj_list = vec![vec![]; adj_list.len()];
    for v in 0..adj_list.len() {
        for &u in &adj_list[v] {
            transposed_adj_list[u as usize].push(v as u32);
        }
    }
    transposed_adj_list
}

fn time_out_sort(adj_list: &Vec<Vec<u32>>) -> Vec<u32> {
    fn dfs(v: u32, adj_list: &Vec<Vec<u32>>, visited: &mut Vec<bool>, order: &mut Vec<u32>) {
        for &u in &adj_list[v as usize] {
            if !visited[u as usize] {
                visited[u as usize] = true;
                dfs(u, adj_list, visited, order);
            }
        }
        order.push(v);
    }

    let n =  adj_list.len();
    let mut visited = vec![false; n];
    let mut order = vec![];
    for v in 0..n {
        if !visited[v]  {
            visited[v] = true;
            dfs(v as u32, adj_list, &mut visited, &mut order);
        }
    }
    order
}

fn connected_components(adj_list: &Vec<Vec<u32>>, order: &Vec<u32>) -> (u32, Vec<u32>) {
    let mut stack = vec![];
    let mut components = vec![0; adj_list.len()];
    let mut visited = vec![false; adj_list.len()];
    let mut component = 0;

    for &v in order {
        if !visited[v as usize] {
            visited[v as usize] = true;
            stack.push(v);
            while let Some(v) = stack.pop() {
                components[v as usize] = component;
                for &u in &adj_list[v as usize] {
                    if !visited[u as usize] {
                        visited[u as usize] = true;
                        stack.push(u);
                    }
                }
            }
            component += 1;
        }
    }
    (component, components)
}

fn strongly_connected_components(adj_list: &Vec<Vec<u32>>) -> (u32, Vec<u32>) {
    let mut order = time_out_sort(adj_list);
    order.reverse();
    connected_components(&transpose(adj_list), &order)
}

fn cheapest_secure_subset(adj_list: &Vec<Vec<u32>>, costs: &[u32]) -> (u64, u32) {
    let (components_n, components) = strongly_connected_components(adj_list);
    let mut component_min_cost = vec![u32::MAX; components_n as usize];
    let mut component_min_cnt = vec![0 as u32; components_n as usize];
    for (v, &component)  in components.iter().enumerate() {
        if component_min_cost[component as usize] > costs[v] {
            component_min_cost[component as usize] =  costs[v];
            component_min_cnt[component as usize] = 1;

        } else if component_min_cost[component as usize] == costs[v] {
            component_min_cnt[component as usize] += 1;
        }
    }
    ( 
        component_min_cost.iter().map(|&cost| cost as u64).sum(),
        component_min_cnt.iter().fold(1 as u64, |acc, &size| (acc * size as u64) % 1000000007) as u32
    )
}

fn input() -> String {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    buffer
}

fn main() {
    let n: u32 = input().trim().parse().unwrap();
    let costs: Vec<u32> = input().trim().split(' ').map(|val| val.parse().unwrap()).collect();
    let m = input().trim().parse().unwrap();
    let edges: Vec<(u32, u32)> = (0..m).map( |_| 
        {
            let line = input();
            let mut iter = line.trim().split(' ').map(|s| s.parse::<u32>().unwrap() - 1);
            (iter.next().unwrap(), iter.next().unwrap())
        }

    ).collect();

    let (min_cost, number_of_ways) = cheapest_secure_subset(&edges_to_adj_list(n, &edges), &costs);
    println!("{} {}", min_cost, number_of_ways);
}
