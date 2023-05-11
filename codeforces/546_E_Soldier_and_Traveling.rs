use std::io;
use std::collections::VecDeque;

fn max_flow(s: u8, t: u8, adj_list: &Vec<Vec<u8>>, capacity: &mut Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    let n = adj_list.len();
    let mut flow = vec![vec![0; n]; n];

    while let Some(parents) = shortest_path(s, t, adj_list, capacity) {
        let add_flow_value = additional_flow_value(t, capacity, &parents);
        let mut current = t;
        while let Some(parent) = parents[current as usize] {
            if flow[current as usize][parent as usize] > 0 {
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

fn additional_flow_value(t: u8, capacity: &Vec<Vec<u8>>, parents: &Vec<Option<u8>>) -> u8 {
    let mut current = t;
    let mut add_flow_value = u8::MAX;
    while let Some(parent) = parents[current as usize] {
        add_flow_value = add_flow_value.min(capacity[parent as usize][current as usize]);
        current = parent;
    }
    add_flow_value
}

fn shortest_path(source: u8, target: u8, adj_list: &Vec<Vec<u8>>, capacity: &Vec<Vec<u8>>) -> Option<Vec<Option<u8>>> {
    let n = adj_list.len();
    let mut queue = VecDeque::from([source]);
    let mut visited = vec![false; n];
    visited[source as usize] = true;
    let mut parents = vec![None; n];

    while let Some(v) = queue.pop_front() {
        if v == target {
            return Some(parents);
        }
        for &u in &adj_list[v as usize] {
            if !visited[u as usize] && capacity[v as usize][u as usize] > 0 {
                visited[u as usize] = true;
                parents[u as usize] = Some(v);
                queue.push_back(u);
            }
        }
    }
    None
}


fn into_network(n: u8, a: &[u8], b: &[u8], edges: &[(u8,u8)]) -> (Vec<Vec<u8>>, Vec<Vec<u8>>) {
    let net_n = n as usize * 2 + 2;
    let mut adj_list = vec![vec![]; net_n];
    let mut capacity = vec![vec![0; net_n]; net_n];
    for v in 0..n {
        let (v_in, v_out) = (v + 2, v + 2 + n);

        adj_list[0].push(v_in);
        adj_list[v_in as usize].push(0);
        capacity[0][v_in as usize] = a[v as usize];

        adj_list[v_in as usize].push(1);
        adj_list[1].push(v_in);
        capacity[v_in as usize][1] = b[v as usize];

        adj_list[v_in as usize].push(v_out);
        adj_list[v_out as usize].push(v_in);
        capacity[v_in as usize][v_out as usize] = a[v as usize];
    }

    for &(v, u) in edges {
        let (v_in, v_out) = (v + 2, v + 2 + n);
        let (u_in, u_out) = (u + 2, u + 2 + n);

        adj_list[v_out as usize].push(u_in);
        adj_list[u_in as usize].push(v_out);
        capacity[v_out as usize][u_in as usize] = u8::MAX;


        adj_list[u_out as usize].push(v_in);
        adj_list[v_in as usize].push(u_out);
        capacity[u_out as usize][v_in as usize] = u8::MAX;
    }
    (adj_list, capacity)
}

fn soldier_and_travel(n: u8, a: &[u8], b: &[u8], edges: &[(u8,u8)])-> Option<Vec<Vec<u8>>> {
    let (adj_list, mut capacity) = into_network(n, a, b, edges);
    let flow = max_flow(0, 1, &adj_list, &mut capacity);

    if (0..n).all(|v| flow[(v + 2) as usize][1] == b[v as usize])  && (0..n).all(|v| flow[0][(v + 2) as usize] == a[v as usize]) {
        return Some(
            (0..n).map(|v| (0..n).map(|u| 
                {
                    let (v_out, u_in) = (v + 2 + n, u + 2);
                    if v == u {
                        a[v as usize] - flow[u_in as usize][v_out as usize]    
                    } else {
                        flow[v_out as usize][u_in as usize]    
                    }
                }
            ).collect()).collect()
        );
    }
    None
}

fn read_numbers() -> Vec<u8> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    buffer.trim().split_ascii_whitespace().map(|word| word.parse().unwrap()).collect()
}

fn main() {
    let v = read_numbers();
    let (n, m) = (v[0], v[1]);
    let a = read_numbers();
    let b = read_numbers();
    let roads = (0..m).map(
        |_| {
            let v = read_numbers() ;
            (v[0] - 1, v[1] - 1)
        }
    ).collect::<Vec<_>>();
    if let Some(matrix) = soldier_and_travel(n, &a, &b, &roads) {
        println!("YES");
        for row in  matrix {
            println!("{}", row.into_iter().map(|num| num.to_string()).collect::<Vec<_>>().join(" "));
        }
    } else {
        println!("NO");
    }
}

#[test]
fn max_flow_test() {
    assert_eq!(
        vec![vec![0, 8], vec![0, 0]],
        max_flow(0, 1, &vec![vec![1], vec![]], &mut vec![vec![0, 8], vec![0, 0]])
    );

    assert_eq!(
        vec![
            vec![0, 2, 1, 0], 
            vec![0, 0, 1, 1], 
            vec![0, 0, 0, 2], 
            vec![0, 0, 0, 0],                 
        ],
        max_flow(
            0, 
            3, 
            &vec![
                vec![1, 2], 
                vec![2, 3],
                vec![3],
                vec![]
            ], 
            &mut vec![
                vec![0, 2, 1, 0], 
                vec![0, 0, 1, 1], 
                vec![0, 0, 0, 3], 
                vec![0, 0, 0, 0],                 
            ]
        )
    );
}
