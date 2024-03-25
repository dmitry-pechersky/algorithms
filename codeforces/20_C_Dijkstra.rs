use std::{
    cmp::Reverse,
    collections::BinaryHeap,
    fmt::Write,
    io::{stdin, Read},
};

fn read_input() -> (u32, Vec<Vec<(u32, u32)>>) {
    let mut buffer = String::new();
    stdin().read_to_string(&mut buffer).unwrap();
    let mut input = buffer.split_whitespace().map(|word| word.parse().unwrap());
    let (n, m) = (input.next().unwrap(), input.next().unwrap());
    let mut adj_list = vec![vec![]; n as usize];
    for _ in 0..m {
        let (v, u, w) = (
            input.next().unwrap() - 1,
            input.next().unwrap() - 1,
            input.next().unwrap(),
        );
        adj_list[v as usize].push((u, w));
        adj_list[u as usize].push((v, w));
    }
    (n, adj_list)
}

fn write_output(path: &[u32]) {
    if path.is_empty() {
        println!("{}", -1);
    } else {
        let mut buffer = String::new();
        for v in path.iter() {
            write!(&mut buffer, "{} ", v + 1).unwrap();
        }
        println!("{}", buffer);
    }
}

fn dijkstra(n: u32, adj_list: &[Vec<(u32, u32)>]) -> Vec<u32> {
    let mut dests = vec![u64::MAX; n as usize];
    let mut parents = vec![0; n as usize];
    dests[0] = 0;
    let mut heap = BinaryHeap::from([(Reverse(0), 0)]);
    while let Some((_, v)) = heap.pop() {
        if v == n - 1 {
            let mut v = v;
            let mut path = vec![v];
            while parents[v as usize] != v {
                v = parents[v as usize];
                path.push(v);
            }
            path.reverse();
            return path;
        }
        for &(u, w) in &adj_list[v as usize] {
            let u_dest = dests[v as usize] + w as u64;
            if u_dest < dests[u as usize] {
                dests[u as usize] = u_dest;
                parents[u as usize] = v;
                heap.push((Reverse(u_dest), u));
            }
        }
    }
    vec![]
}

fn main() {
    let (n, adj_list) = read_input();
    write_output(&dijkstra(n, &adj_list));
}
